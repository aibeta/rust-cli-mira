use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};
use enigo::{
    Direction::{Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use std::thread;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device_state = DeviceState::new();
    let mut enigo = Enigo::new(&Settings::default())?;

    let mut last_activity = Instant::now();
    let timeout = Duration::from_secs(30);

    println!("Monitoring for input events. Will simulate key press after 30 seconds of inactivity.");
    println!("Press Ctrl+C to exit.");

    use std::process::Command;

    let _ = Command::new("osascript")
        .arg("-e")
        .arg(r#"display dialog "休5分钟吧，电脑此时会被锁定" buttons {"OK"} default button "OK""#)
        .status();


    loop {
        let mouse: MouseState = device_state.get_mouse();
        let keys: Vec<Keycode> = device_state.get_keys();

        // Check if there's any mouse movement or key press
        if mouse.button_pressed[0] || mouse.button_pressed[1] || mouse.button_pressed[2] || !keys.is_empty() {
            last_activity = Instant::now();
        }

        // If 30 seconds have passed since last activity, simulate a key press
        if last_activity.elapsed() >= timeout {
            println!("Simulating Control+Alt+Q...");
            enigo.key(Key::Control, Press)?;
            enigo.key(Key::Alt, Press)?;
            enigo.key(Key::Unicode('q'), Press)?;
            enigo.key(Key::Control, Release)?;
            enigo.key(Key::Alt, Release)?;
            enigo.key(Key::Unicode('q'), Release)?;
            last_activity = Instant::now();
        }

        // Small sleep to prevent high CPU usage
        thread::sleep(Duration::from_millis(100));
    }
}
