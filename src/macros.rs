#[macro_export]
macro_rules! with_static_ref_option {
    ([$source:expr => $name:ident] $block:block or $handle_none:block ) => {
        $source.with(|ref_cell| {
            let ref_pointer = &mut *ref_cell.borrow_mut();

            if ref_pointer.is_none() {
                $handle_none
                return;
            }

            let $name = ref_pointer.as_mut().unwrap();

            $block
        });
    };

    (let $name:ident = { $source:expr } or $handle_none:block;$($block:stmt)*) => {
        $source.with(|ref_cell| {
            let ref_pointer = &mut *ref_cell.borrow_mut();

            if ref_pointer.is_none() {
                $handle_none
                return;
            }

            let $name = ref_pointer.as_mut().unwrap();

            $($block)*
        });
    }
}

#[macro_export]
macro_rules! with_audio_context {
    ([$static_ref:expr => $context_name:ident] $block:expr) => {
        with_static_ref_option!([$static_ref => audio_device] {
            let mut audio_context_lock = audio_device.lock();
            let $context_name = audio_context_lock.deref_mut();

            $block
        } or {
            println!("Sound: no audio device initialized");
        });
    }
}

#[macro_export]
macro_rules! with_break {
    ($( $block:stmt )*) => {
        loop {
            $( $block )*
            break;
        }
    };
}
