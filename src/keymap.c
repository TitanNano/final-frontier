/*
  Hatari - main.c

  This file is distributed under the GNU Public License, version 2 or at
  your option any later version. Read the file gpl.txt for details.

  Here we process a key press and the remapping of the scancodes.
*/

#include "lib_main.h"
#include "keymap.h"
#include "input.h"
#include "shortcut.h"
#include "screen.h"


/*-----------------------------------------------------------------------*/
/*
  Remap table of PC keys to ST scan codes, -ve is invalid key (ie doesn't occur on ST)

  PC Keyboard:-

    Esc  F1  F2  F3  F4    F5  F6  F7  F8    F9  F10 F11 F12       Print Scroll Pause

     1   59  60  61  62    63  64  65  66    67  68  87  88                70     69


�  !   "   �   $   %   ^   &   *   (   )   _   +                                 Page
`  1   2   3   4   5   6   7   8   9   0   -   =   <-               Ins   Home    Up

41 2   3   4   5   6   7   8   9   10  11  12  13  14               82     71     73
                                                                    --     --     --
                                                       |
                                             {   }     |                         Page
Tab  Q   W   E   R   T   Y   U   I   O   P   [   ] <----            Del   End    Down

15   16  17  18  19  20  21  22  23  24  25  26  27  28             83     79     81
                                                                    --     --     --

                                           :   @   ~                       ^
Caps   A   S   D   F   G   H   J   K   L   ;   '   #                       |

58     30  31  32  33  34  35  36  37  38  39  40  43                      72
                                                                           --

^   |                               <   >   ?   ^
|   \   Z   X   C   V   B   N   M   ,   .   /   |                     <-   |   ->

42  86  44  45  46  47  48  49  50  51  52  53  54                    75   80  77
                                                                      --   --  --

Ctrl  Alt          SPACE             Alt Gr      Ctrl

 29    56            57                56         29
                                                  --

  And:-

Num
Lock   /    *    -

69     53   55   74
--     --

 7    8     9    +
Home  ^   Pg Up

71    72    73   78


4     5     6
<-          ->

75    76    77


1     2     3
End   |   Pg Dn  Enter

79    70    81    28
                  --

0     .
Ins   Del

82    83


*/

/* This table is used to translate a symbolic keycode to the (SDL) scancode */
static Uint8 SdlSymToSdlScan[SDL_NUM_SCANCODES];


/* List of ST scan codes to NOT de-bounce when running in maximum speed */
static char DebounceExtendedKeys[] =
{
  0x1d,  /* CTRL */
  0x2a,  /* Left SHIFT */
  0x01,  /* ESC */
  0x38,  /* ALT */
  0x36,  /* Right SHIFT */
  0      /* term */
};



/*-----------------------------------------------------------------------*/
/*
  Initialization.
*/
void Keymap_Init(void)
{
  memset (SdlSymToSdlScan, 0, sizeof(SdlSymToSdlScan));
}

/*-----------------------------------------------------------------------*/
/*
  Heuristic analysis to find out the obscure scancode offset.
  This clever code has been taken from the emulator Aranym. (cheers!)
*/
static int Keymap_FindScanCodeOffset(SDL_Keysym* keysym)
{
  int offset = -1;    /* uninitialized scancode offset */
  int scanPC = keysym->scancode; // Keymap_GetPCScanCode(keysym);

  if(scanPC == 0)  return -1;  /* Ignore illegal scancode */

  switch(keysym->sym)
  {
    case SDLK_ESCAPE:  offset = scanPC - 0x01; break;
    case SDLK_1:  offset = scanPC - 0x02; break;
    case SDLK_2:  offset = scanPC - 0x03; break;
    case SDLK_3:  offset = scanPC - 0x04; break;
    case SDLK_4:  offset = scanPC - 0x05; break;
    case SDLK_5:  offset = scanPC - 0x06; break;
    case SDLK_6:  offset = scanPC - 0x07; break;
    case SDLK_7:  offset = scanPC - 0x08; break;
    case SDLK_8:  offset = scanPC - 0x09; break;
    case SDLK_9:  offset = scanPC - 0x0a; break;
    case SDLK_0:  offset = scanPC - 0x0b; break;
    case SDLK_BACKSPACE:  offset = scanPC - 0x0e; break;
    case SDLK_TAB:  offset = scanPC - 0x0f; break;
    case SDLK_RETURN:  offset = scanPC - 0x1c; break;
    case SDLK_SPACE:  offset = scanPC - 0x39; break;
    case SDLK_q:  offset = scanPC - 0x10; break;
    case SDLK_w:  offset = scanPC - 0x11; break;
    case SDLK_e:  offset = scanPC - 0x12; break;
    case SDLK_r:  offset = scanPC - 0x13; break;
    case SDLK_t:  offset = scanPC - 0x14; break;
    case SDLK_y:  offset = scanPC - 0x15; break;
    case SDLK_u:  offset = scanPC - 0x16; break;
    case SDLK_i:  offset = scanPC - 0x17; break;
    case SDLK_o:  offset = scanPC - 0x18; break;
    case SDLK_p:  offset = scanPC - 0x19; break;
    case SDLK_a:  offset = scanPC - 0x1e; break;
    case SDLK_s:  offset = scanPC - 0x1f; break;
    case SDLK_d:  offset = scanPC - 0x20; break;
    case SDLK_f:  offset = scanPC - 0x21; break;
    case SDLK_g:  offset = scanPC - 0x22; break;
    case SDLK_h:  offset = scanPC - 0x23; break;
    case SDLK_j:  offset = scanPC - 0x24; break;
    case SDLK_k:  offset = scanPC - 0x25; break;
    case SDLK_l:  offset = scanPC - 0x26; break;
    case SDLK_z:  offset = scanPC - 0x2c; break;
    case SDLK_x:  offset = scanPC - 0x2d; break;
    case SDLK_c:  offset = scanPC - 0x2e; break;
    case SDLK_v:  offset = scanPC - 0x2f; break;
    case SDLK_b:  offset = scanPC - 0x30; break;
    case SDLK_n:  offset = scanPC - 0x31; break;
    case SDLK_m:  offset = scanPC - 0x32; break;
    case SDLK_CAPSLOCK:  offset = scanPC - 0x3a; break;
    case SDLK_LSHIFT:  offset = scanPC - 0x2a; break;
    case SDLK_LCTRL:  offset = scanPC - 0x1d; break;
    case SDLK_LALT:  offset = scanPC - 0x38; break;
    case SDLK_F1:  offset = scanPC - 0x3b; break;
    case SDLK_F2:  offset = scanPC - 0x3c; break;
    case SDLK_F3:  offset = scanPC - 0x3d; break;
    case SDLK_F4:  offset = scanPC - 0x3e; break;
    case SDLK_F5:  offset = scanPC - 0x3f; break;
    case SDLK_F6:  offset = scanPC - 0x40; break;
    case SDLK_F7:  offset = scanPC - 0x41; break;
    case SDLK_F8:  offset = scanPC - 0x42; break;
    case SDLK_F9:  offset = scanPC - 0x43; break;
    case SDLK_F10:  offset = scanPC - 0x44; break;
    default:  break;
  }

  if (offset != -1)
  {
    fprintf(stderr, "Detected scancode offset = %d (key: '%s' with scancode $%02x)\n",
            offset, SDL_GetKeyName(keysym->sym), scanPC);
  }

  return offset;
}


/*-----------------------------------------------------------------------*/
/*
  Map PC scancode to ST scancode.
  This code was heavily inspired by the emulator Aranym. (cheers!)
*/
static char Keymap_PcToStScanCode(SDL_Keysym* keysym)
{
  static int offset = -1;    /* uninitialized scancode offset */

  switch(keysym->sym)
  {
    /* Numeric Pad */
    /* note that the numbers are handled in Keymap_GetKeyPadScanCode()! */
    case SDLK_KP_DIVIDE:   return 0x65;  /* Numpad / */
    case SDLK_KP_MULTIPLY: return 0x66;  /* NumPad * */
    case SDLK_KP_MINUS:    return 0x4a;  /* NumPad - */
    case SDLK_KP_PLUS:     return 0x4e;  /* NumPad + */
    case SDLK_KP_PERIOD:   return 0x71;  /* NumPad . */
    case SDLK_KP_ENTER:    return 0x72;  /* NumPad Enter */

    /* Special Keys */
    /*case SDLK_F11:  return 0x62;*/  /* F11 => Help */
    /*case SDLK_F12:  return 0x61;*/  /* F12 => Undo */
    case SDLK_HOME:   return 0x47;  /* Home */
    case SDLK_END:    return 0x60;  /* End => "<>" on German Atari kbd */
    case SDLK_UP:     return 0x48;  /* Arrow Up */
    case SDLK_LEFT:   return 0x4b;  /* Arrow Left */
    case SDLK_RIGHT:  return 0x4d;  /* Arrow Right */
    case SDLK_DOWN:   return 0x50;  /* Arrow Down */
    case SDLK_INSERT: return 0x52;  /* Insert */
    case SDLK_DELETE: return 0x53;  /* Delete */
    case SDLK_LESS:   return 0x60;  /* "<" */

    /* Map Right Alt/Alt Gr/Control to the Atari keys */
    case SDLK_ESCAPE:     return 0x01;
    case SDLK_RCTRL:      return 0x1d;  /* Control */
    case SDLK_RALT:       return 0x38;  /* Alternate */
    case SDLK_1:          return 0x02;
    case SDLK_2:          return 0x03;
    case SDLK_3:          return 0x04;
    case SDLK_4:          return 0x05;
    case SDLK_5:          return 0x06;
    case SDLK_6:          return 0x07;
    case SDLK_7:          return 0x08;
    case SDLK_8:          return 0x09;
    case SDLK_9:          return 0x0A;
    case SDLK_0:          return 0x0B;
    case SDLK_BACKSPACE:  return 0x0E;
    case SDLK_TAB:        return 0x0F;
    case SDLK_RETURN:     return 0x1C;
    case SDLK_SPACE:      return 0x39;
    case SDLK_q:          return 0x10;
    case SDLK_w:          return 0x11;
    case SDLK_e:          return 0x12;
    case SDLK_r:          return 0x13;
    case SDLK_t:          return 0x14;
    case SDLK_y:          return 0x15;
    case SDLK_u:          return 0x16;
    case SDLK_i:          return 0x17;
    case SDLK_o:          return 0x18;
    case SDLK_p:          return 0x19;
    case SDLK_a:          return 0x1E;
    case SDLK_s:          return 0x1F;
    case SDLK_d:          return 0x20;
    case SDLK_f:          return 0x21;
    case SDLK_g:          return 0x22;
    case SDLK_h:          return 0x23;
    case SDLK_j:          return 0x24;
    case SDLK_k:          return 0x25;
    case SDLK_l:          return 0x26;
    case SDLK_z:          return 0x2C;
    case SDLK_x:          return 0x2D;
    case SDLK_c:          return 0x2e;
    case SDLK_v:          return 0x2f;
    case SDLK_b:          return 0x30;
    case SDLK_n:          return 0x31;
    case SDLK_m:          return 0x32;
    case SDLK_CAPSLOCK:   return 0x3A;
    case SDLK_LSHIFT:     return 0x2A;
    case SDLK_LCTRL:      return 0x1D;
    case SDLK_LALT:       return 0x38;
    case SDLK_F1:         return 0x3B;
    case SDLK_F2:         return 0x3C;
    case SDLK_F3:         return 0x3D;
    case SDLK_F4:         return 0x3E;
    case SDLK_F5:         return 0x3F;
    case SDLK_F6:         return 0x40;
    case SDLK_F7:         return 0x41;
    case SDLK_F8:         return 0x42;
    case SDLK_F9:         return 0x43;
    case SDLK_F10:        return 0x44;

    default:
    {
      /* Process remaining keys: assume that it's PC101 keyboard
       * and that it is compatible with Atari ST keyboard (basically
       * same scancodes but on different platforms with different
       * base offset (framebuffer = 0, X11 = 8).
       * Try to detect the offset using a little bit of black magic.
       * If offset is known then simply pass the scancode. */
      int scanPC = keysym->scancode; // Keymap_GetPCScanCode(keysym);
      if (offset == -1)
      {
        offset = Keymap_FindScanCodeOffset(keysym);
      }

      if (offset >= 0)
      {
        /* offset is defined so pass the scancode directly */
        return (scanPC - offset);
      }
      else
      {
        fprintf(stderr, "Unknown key: scancode = %d ($%02x), keycode = '%s' ($%02x)\n",
                scanPC, scanPC, SDL_GetKeyName(keysym->sym), keysym->sym);
	      fprintf(stderr,"trying offset 8 (the most likely !)\n");
        return (scanPC - 8);
      }
    }
  }
}


/*-----------------------------------------------------------------------*/
/*
  Remap a keypad key to ST scan code. We use a separate function for this
  so that we can easily toggle between number and cursor mode with the
  numlock key.
*/
static char Keymap_GetKeyPadScanCode(SDL_Keysym* pKeySym)
{
  if(SDL_GetModState() & KMOD_NUM)
  {
    switch(pKeySym->sym)
    {
      case SDLK_KP_0:  return 0x70;  /* NumPad 0 */
      case SDLK_KP_1:  return 0x6d;  /* NumPad 1 */
      case SDLK_KP_2:  return 0x6e;  /* NumPad 2 */
      case SDLK_KP_3:  return 0x6f;  /* NumPad 3 */
      case SDLK_KP_4:  return 0x6a;  /* NumPad 4 */
      case SDLK_KP_5:  return 0x6b;  /* NumPad 5 */
      case SDLK_KP_6:  return 0x6c;  /* NumPad 6 */
      case SDLK_KP_7:  return 0x67;  /* NumPad 7 */
      case SDLK_KP_8:  return 0x68;  /* NumPad 8 */
      case SDLK_KP_9:  return 0x69;  /* NumPad 9 */
      default:  break;
    }
  }
  else
  {
    switch(pKeySym->sym)
    {
      case SDLK_KP_0:  return 0x70;  /* NumPad 0 */
      case SDLK_KP_1:  return 0x6d;  /* NumPad 1 */
      case SDLK_KP_2:  return 0x50;  /* Cursor down */
      case SDLK_KP_3:  return 0x6f;  /* NumPad 3 */
      case SDLK_KP_4:  return 0x4b;  /* Cursor left */
      case SDLK_KP_5:  return 0x50;  /* Cursor down (again?) */
      case SDLK_KP_6:  return 0x4d;  /* Cursor right */
      case SDLK_KP_7:  return 0x52;  /* Insert - good for Dungeon Master */
      case SDLK_KP_8:  return 0x48;  /* Cursor up */
      case SDLK_KP_9:  return 0x47;  /* Home - again for Dungeon Master */
      default:  break;
    }
  }

  return -1;
}


/*-----------------------------------------------------------------------*/
/*
  Remap SDL Key to ST Scan code
*/
char Keymap_RemapKeyToSTScanCode(SDL_Keysym* pKeySym)
{
  //if(pKeySym->sym >= SDLK_LAST)  return -1;  /* Avoid illegal keys */

  /* Check for keypad first so we can handle numlock */
    if(pKeySym->sym >= SDLK_KP_0 && pKeySym->sym <= SDLK_KP_9)
    {
      return Keymap_GetKeyPadScanCode(pKeySym);
    }

    /* We sometimes enter here with an illegal (=0) scancode, so we keep
     * track of the right scancodes in a table and then use a value from there.
     */
    if(pKeySym->scancode != 0)
    {
      SdlSymToSdlScan[pKeySym->sym] = pKeySym->scancode;
    }
    else
    {
      pKeySym->scancode = SdlSymToSdlScan[pKeySym->sym];
      if(pKeySym->scancode == 0)
        fprintf(stderr, "Warning: Key scancode is 0!\n");
    }

    fprintf(stderr, "Keypress: Key scancode is %i\n", pKeySym->scancode);

    return Keymap_PcToStScanCode(pKeySym);
}


/*-----------------------------------------------------------------------*/
/*
  Scan list of keys to NOT de-bounce when running in maximum speed, eg ALT,SHIFT,CTRL etc...
  Return TRUE if key requires de-bouncing
*/
BOOL Keymap_DebounceSTKey(char STScanCode)
{
  int i=0;

    /* We should de-bounce all non extended keys, eg leave ALT,SHIFT,CTRL etc... held */
    while (DebounceExtendedKeys[i])
    {
      if (STScanCode==DebounceExtendedKeys[i])
        return(FALSE);
      i++;
    }

    /* De-bounce key */
    return(TRUE);

  /* Do not de-bounce key */
  return(FALSE);
}


/*-----------------------------------------------------------------------*/
/*
  Debounce any PC key held down if running with key repeat disabled
  This is called each ST frame, so keys get held down for one VBL which is enough for 68000 code to scan
*/
void Keymap_DebounceAllKeys(void)
{
  /* Return if we aren't in maximum speed or have not disabled key repeat */
     return;

}


/*-----------------------------------------------------------------------*/
/*
  User press key down
*/
void Keymap_KeyDown(SDL_Keysym *sdlkey)
{
  BOOL bPreviousKeyState;
  char STScanCode;
  int symkey = sdlkey->sym;
  //int scankey = sdlkey->scancode;
  int modkey = sdlkey->mod;

  /*fprintf(stderr, "keydown: sym=%i scan=%i mod=$%x\n",symkey, scankey, modkey);*/

  /* Handle special keys */
  if(symkey == SDLK_MODE || symkey == SDLK_LGUI || symkey == SDLK_NUMLOCKCLEAR)
  {
    /* Ignore modifier keys that aren't passed to the ST */
    return;
  }
  else if(symkey == SDLK_F11 || symkey == SDLK_F12)
  {
    ShortCutKey.Key = symkey;
    return;
  }

  /* Set down */
  bPreviousKeyState = input.key_states[symkey];
  input.key_states[symkey] = TRUE;

  /* If pressed short-cut key, retain keypress until safe to execute (start of VBL) */
  if((modkey&KMOD_MODE) || (modkey&KMOD_RGUI) || (modkey&KMOD_CTRL))
  {
    ShortCutKey.Key = symkey;
    if( modkey&(KMOD_LCTRL|KMOD_RCTRL) )  ShortCutKey.bCtrlPressed = TRUE;
    if( modkey&(KMOD_LSHIFT|KMOD_RSHIFT) )  ShortCutKey.bShiftPressed = TRUE;
  }
  else
  {
    STScanCode = Keymap_RemapKeyToSTScanCode(sdlkey);
    if(STScanCode != (char)-1)
    {
      if(!bPreviousKeyState)
        Input_PressSTKey(STScanCode, TRUE);
    }
  }
}


/*-----------------------------------------------------------------------*/
/*
  User released key
*/
void Keymap_KeyUp(SDL_Keysym *sdlkey)
{
  char STScanCode;
  int symkey = sdlkey->sym;
  //int scankey = sdlkey->scancode;

  /*fprintf(stderr, "keyup: sym=%i scan=%i mod=$%x\n",symkey, scankey, modkey);*/

  /* Handle special keys */
  if(symkey == SDLK_MODE || symkey == SDLK_LGUI || symkey == SDLK_NUMLOCKCLEAR)
  {
    /* Ignore modifier keys that aren't passed to the ST */
    return;
  }
  else if(symkey == SDLK_CAPSLOCK)
  {
    /* Simulate another capslock key press */
    Input_PressSTKey(0x3A, TRUE);
  }
  else if(symkey == SDLK_F11 || symkey == SDLK_F12)
  {
    return;
  }

  /* Release key (only if was pressed) */
  if(input.key_states[symkey])
  {
    STScanCode = Keymap_RemapKeyToSTScanCode(sdlkey);
    if(STScanCode != (char)-1)
    {
      Input_PressSTKey(STScanCode,FALSE);
    }
  }

  input.key_states[symkey] = FALSE;
}
