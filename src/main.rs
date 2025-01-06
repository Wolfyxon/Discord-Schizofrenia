#![windows_subsystem = "windows"]

use std::{thread, time::Duration};

use rand::Rng;
use soloud::{audio, AudioExt, LoadExt, Soloud};

fn main() {
    let mut wav = audio::Wav::default();
    let mut rng = rand::thread_rng();

    wav.load_mem(include_bytes!("./ping.ogg")).unwrap();
    
    loop {
        let delay: f32 = rng.gen_range(60.0..(60.0 * 60.0));

        thread::sleep(Duration::from_secs_f32(delay));
        
        let sl = Soloud::default().unwrap();
        sl.play(&wav);
        
        thread::sleep(Duration::from_secs_f64(wav.length()));
    };
}
