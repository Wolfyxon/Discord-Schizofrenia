#![windows_subsystem = "windows"]

use std::{thread, time::Duration};

use soloud::{audio, AudioExt, LoadExt, Soloud};

fn main() {
    let sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();

    wav.load_mem(include_bytes!("./ping.ogg")).unwrap();
    
    loop {
        thread::sleep(Duration::from_secs_f32(2.0));
        sl.play(&wav);
    };
}
