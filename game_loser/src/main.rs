#![windows_subsystem = "windows"]

use notify_rust::{Notification, Urgency::Critical, CloseReason};
use std::thread;
use std::time;

fn main() {
    let sleep_time = time::Duration::from_mins(15);

    loop {
        thread::sleep(sleep_time);
        let notification = Notification::new()
            .summary("News from Bridgette")
            .body("I lost the game :(")
            .urgency(Critical)
            .timeout(0)
            .show()
            .unwrap();
        notification.on_close(|| close_application());
    }
}

fn close_application() {
    panic!("Sorry")
}