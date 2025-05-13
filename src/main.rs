use chrono::{Local, Timelike};
use enigo::{
    Direction::{Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut enigo = Enigo::new(&Settings::default())?;

    loop {
        // If 30 seconds have passed since last activity, simulate a key press

        let current_second = Local::now().second();
        if current_second % 30 == 0 {
            println!("Simulating Control+Alt+Q...");
            enigo.key(Key::Control, Press)?;
            enigo.key(Key::Alt, Press)?;
            enigo.key(Key::Unicode('q'), Press)?;
            enigo.key(Key::Control, Release)?;
            enigo.key(Key::Alt, Release)?;
            enigo.key(Key::Unicode('q'), Release)?;
        }

        // Small sleep to prevent high CPU usage
        thread::sleep(Duration::from_millis(1000));
    }
}

