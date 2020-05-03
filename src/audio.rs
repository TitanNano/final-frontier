use std::fs::File;
use std::cell::RefCell;
use std::ops::DerefMut;

use rand::Rng;
use lewton::inside_ogg::OggStreamReader;
use sdl2::audio::{
    AudioCallback, AudioSpecWAV, AudioFormat, AudioDevice, AudioFormatNum
};

use crate::{ SdlContext, GameConfig };

type StaticAudioDeviceRef = RefCell<Option<AudioDevice<Callback>>>;


/* Converted frontier SFX to wav samples. */
static MAX_SAMPLES: i32	= 33;
static SND_FREQ: i32 = 22050;
static MAX_CHANNELS: usize = 4;

thread_local! {
    static AUDIO_DEVICE: StaticAudioDeviceRef = RefCell::default();
}

#[derive(Clone, PartialEq)]
struct WavStream {
    buffer: Vec<u8>,
    should_loop: i32, // -1 no loop, otherwise specifies loop start pos
}

impl WavStream {
    fn new(buffer: Vec<u8>, should_loop: i32) -> Self {
        Self {
            buffer,
            should_loop,
        }
    }
}

#[derive(Clone)]
struct WavChannel {
    buffer_pos: usize,
    should_loop: i32,
    buffer_ref: usize
}

impl WavChannel {
    fn buffer_pos(&self) -> usize {
        self.buffer_pos
    }

    fn set_buffer_pos(&mut self, pos: usize) {
        self.buffer_pos = pos;
    }

    fn should_loop(&self) -> i32 {
        self.should_loop
    }

    fn buffer<'a>(&self, context: &'a Callback) -> &'a Vec<u8> {
        &context.sfx_list[self.buffer_ref].buffer
    }

    fn from_stream(context: &[WavStream], stream: &WavStream) -> Self {
        let index = context.iter().position(|item| item == stream).expect("Context doees not contain stream!");

        Self {
            buffer_pos: 0,
            should_loop: stream.should_loop,
            buffer_ref: index
        }
    }
}

struct Callback {
    wav_channels: Vec<Option<WavChannel>>,
    sfx_list: Vec<WavStream>,
    music_file: Option<OggStreamReader<File>>,
    enabled_tracks: usize,
    music_mode: isize,
}

impl Callback {
    fn new(sfx_list: Vec<WavStream>) -> Self {
        let wav_channels = vec![None; MAX_CHANNELS];

        Self {
            wav_channels,
            sfx_list,
            music_file: None,
            enabled_tracks: 0,
            music_mode: 0,
        }
    }

    fn set_wav_channel(&mut self, channel: usize, wav_channel: WavChannel) {
        self.wav_channels[channel] = Some(wav_channel)
    }

    fn wav_channels(&self) -> Vec<Option<&WavChannel>> {
        self.wav_channels.iter()
            .map(|option| option.as_ref())
            .collect()
    }

    fn wav_channels_mut(&mut self) -> Vec<Option<&mut WavChannel>> {
        self.wav_channels.iter_mut()
            .map(|option| option.as_mut())
            .collect()
    }

    fn clear_wav_channel(&mut self, channel: usize) {
        self.wav_channels[channel] = None;
    }
}

trait Reset<T> {
    fn reset(&mut self, value: Option<T>);
}

impl Reset<i16> for [i16] {
    fn reset(&mut self, value: Option<i16>) {
        for byte in self.iter_mut() {
            *byte = value.unwrap_or(0);
        }
    }
}


impl AudioCallback for Callback {
    type Channel = i16; /* 8 Bit unsigned audio format */

    fn callback(&mut self, dest_buffer: &mut [Self::Channel]) {
        let playing = self.wav_channels.iter().any(|channel| channel.is_some());

        dest_buffer.reset(Some(Self::Channel::SILENCE));

        if let Some(ref mut music_file) = self.music_file {
            let mut i = 0;

            while i < dest_buffer.len() {
                let sample = music_file.read_dec_packet_itl().expect("vorbis read");

                if let Some(sample) = sample {
                    dest_buffer[i..(i + sample.len())].copy_from_slice(&sample);

                    i += sample.len();

                    continue;
                }

                /* end of stream */
                println!("ogg stream ended.");

                if self.music_mode == -1 {
                    play_music_track(self, rand_tracknum(self));
                    break;
                }

                stop_music_track(self);
                break;
            }
        }

        if !playing {
            return;
        }

        for i in (0..dest_buffer.len()).step_by(2) {
            let mut sample: i16 = 0;

            for j in 0..MAX_CHANNELS {
                let wav_channels = self.wav_channels();

                if wav_channels[j].is_none() {
                    continue;
                }

                let channel = wav_channels[j].unwrap();
                let buffer = channel.buffer(self);
                let buffer_pos = channel.buffer_pos();
                let buffer_len = buffer.len();

                let byte_a = buffer[buffer_pos] as u16;
                let byte_b = buffer[buffer_pos+1] as u16;

                sample += (byte_a | byte_b << 8) as i16;

                { // create scope to contain mutable ref
                    let mut wav_channels = self.wav_channels_mut();
                    let mut channel = wav_channels[j].as_mut().unwrap();

                    channel.set_buffer_pos(buffer_pos + 2);

                    if channel.buffer_pos() < buffer_len {
                        continue;
                    }

                    /* end of sample. either loop or terminate */
                    if channel.should_loop() != -1 {
                        channel.buffer_pos = channel.should_loop as usize;
                        continue;
                    }
                }

                self.clear_wav_channel(j);
            }

            /* stereo! */
            dest_buffer[i] += sample;
            dest_buffer[i+1] += sample;
        }
    }
}

pub fn init(context: &SdlContext, config: &GameConfig) {
    /* Is enabled? */
    if config.nosound {
        /* Stop any sound access */
        println!("Sound: Disabled");
        return;
    }

    let mut sfx_list = vec!();
    let num_audio_devices = context.audio().num_audio_playback_devices();

    if num_audio_devices.is_none() {
        println!("Sound: Disabled, not available!");
        return;
    }

    let num_audio_devices = num_audio_devices.unwrap();

    if num_audio_devices == 0 {
        println!("Sound: Disabled, no audio device found!");
        return;
    }

    /* Set up SDL audio: */
    let desired_spec = sdl2::audio::AudioSpecDesired {
        freq: Some(SND_FREQ),
        channels: Some(2), /* Mono */
        samples: Some(1024), /* Buffer size */
    };


    for i in 0..MAX_SAMPLES {
        let filename = format!("sfx/sfx_{:02}.wav", i);
        let wave_load_state = AudioSpecWAV::load_wav(&filename);

        if let Err(error) = wave_load_state {
            println!("Error loading WAV: {}\n", error);
            continue;
        }

        let sfx_spec = wave_load_state.unwrap();

        let sfx_buffer = check_sample_format(&sfx_spec, &filename);

        if sfx_buffer.is_none() {
            continue;
        }

        let sfx_buffer = sfx_buffer.unwrap();

        let sfx = match i {
            19 => { // hyperspace
                WavStream::new(sfx_buffer, SND_FREQ) // loop to about 0.5 sec in
            },

            23 => { // and 23 (noise) loop
                WavStream::new(sfx_buffer, 0) // loop to about 0.5 sec in
            },

            _ => {
                WavStream::new(sfx_buffer, -1)

            }
        };

        sfx_list.push(sfx);
    }

    let audio_device_result = context.audio().open_playback(None, &desired_spec, |_spec| Callback::new(sfx_list));

    if let Err(error) = audio_device_result {
        println!("Sound: {}", error);
        return;
    }

    let audio_device = audio_device_result.unwrap();

    /* And begin */
    enable_audio(&audio_device, true);

    AUDIO_DEVICE.with(|audio_device_ref_cell| {
        *audio_device_ref_cell.borrow_mut() = Some(audio_device);
    });
}

/*
* Loaded samples must be SND_FREQ, 16-bit signed. Reject
* other frequencies but convert 8-bit unsigned.
*/
fn check_sample_format(spec: &AudioSpecWAV, filename: &str) -> Option<Vec<u8>> {

    if spec.freq != SND_FREQ {
        println!("Sample {} is the wrong sample rate (wanted {}Hz). Ignoring.\n", filename, SND_FREQ);
        return None;
    }

    match spec.format {
        AudioFormat::U8 => {
            let old_buffer = spec.buffer();
            let new_buffer: Vec<u8> = old_buffer.iter()
                .map(|byte| {
                    let large_byte = (((byte ^ 128) as i8) as i16) << 8;

                    vec!((large_byte & 0x00FF) as u8, ((large_byte as u16 & 0xFF00) >> 8) as u8)
                })
                .collect::<Vec<Vec<u8>>>()
                .concat();

            Some(new_buffer)
        },

        AudioFormat::S16LSB => {
            Some(spec.buffer().to_owned())
        },

        _ => {
            println!("Sample {} is not 16-bit-signed or 8-bit unsigned. Ignoring.", filename);
            None
        }
    }
}

fn enable_audio(audio_device: &AudioDevice<Callback>, enabled: bool) {
    if !enabled {
        return audio_device.pause();
    }

    audio_device.resume();
}

fn play_music_track(playback_context: &mut Callback, track: usize) {
	let file_path = format!("music/{:02}.ogg", track);
	let file = File::open(&file_path).expect(&format!("Can't open file {}", &file_path));
    let reader = OggStreamReader::new(file).expect("vorbis open");

    playback_context.music_file = Some(reader);
}

fn rand_tracknum(playback_context: &Callback) -> usize {
    if playback_context.enabled_tracks == 0 {
        return 999;
    }

    let mut rng = rand::thread_rng();
    let mut track;

    loop {
        let rand = rng.gen::<usize>();
    	track = rand % 8;

        println!("chose track: {} {}", rand, track);

        if (playback_context.enabled_tracks & (1<<track)) != 0 {
            break;
        }
    }

    track
}

fn stop_music_track(playback_context: &mut Callback) {
    playback_context.music_file = None;
}

pub fn play_sfx(sfx_index: usize, channel: usize) {
    with_audio_context!([AUDIO_DEVICE => audio_context] {
        println!("playing sfx {:02}", sfx_index);

        audio_context.set_wav_channel(channel, WavChannel::from_stream(&audio_context.sfx_list, &audio_context.sfx_list[sfx_index]));
    });
}

pub fn is_music_playing() -> bool {
    let mut is_playing = false;

    with_audio_context!([AUDIO_DEVICE => audio_context] {
        is_playing = audio_context.music_file.is_some();
    });

    is_playing
}

/* music_mode:
 * -2 = play random track once
 * -1 = play random tracks continuously
 * 0+ = play specific track once
 *
 * enabled_tracks: is a mask of enabled tracks
 */
pub fn play_music(music_mode: isize, enabled_tracks: usize) {
    with_audio_context!([AUDIO_DEVICE => audio_context] {
        let mut enabled_tracks = enabled_tracks;

        let track = match music_mode {
            -2 | -1 => {
                // hyperspace and battle music
                // don't play blue danube or reward music
                enabled_tracks &= !0x40;
    			enabled_tracks &= !0x80;

                rand_tracknum(audio_context)
            },
            _ => music_mode as usize
        };

        audio_context.music_mode = music_mode;
        audio_context.enabled_tracks = enabled_tracks;

        play_music_track(audio_context, track);
    });
}

pub fn stop_music() {
    with_audio_context!([AUDIO_DEVICE => audio_context] {
        stop_music_track(audio_context);
    });
}
