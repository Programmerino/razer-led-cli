mod board;
mod driver_sysfs;

use std::{io::{self, BufRead}, process::exit, panic};

use sysinfo::{System, SystemExt, ProcessExt};

fn resume_daemon(pid: i32) {
    println!("Resuming daemon");
    nix::sys::signal::kill(
        nix::unistd::Pid::from_raw(pid),
        nix::sys::signal::Signal::SIGCONT,
    ).unwrap();
    exit(0);
}

fn main() {
    let s = System::new_all();
    let daemon_pid = s
                                    .process_by_name("daemon")
                                    .iter()
                                    .filter(|p| p.exe().to_string_lossy().contains("razercontrol"))
                                    .map(|p| p.pid())
                                    .next();

    match daemon_pid {
        Some(x) => {
            println!("Daemon was running, temporarily pausing it");
            nix::sys::signal::kill(
                nix::unistd::Pid::from_raw(x),
                nix::sys::signal::Signal::SIGSTOP,
            ).unwrap();
            ctrlc::set_handler(move || {
                resume_daemon(x)
            })
            .expect("Error setting Ctrl-C handler");
        },
        None => {}
    }
    
    let result = panic::catch_unwind(|| {
        let mut k = board::KeyboardData::new();
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let [r, g, b] = parse_hex_color(&line.unwrap());
            k.set_kbd_colour(r, g, b);
            k.update_kbd();
        }
    });
    
    match daemon_pid {
        Some(x) => {
            resume_daemon(x);
        },
        None => {}
    }

    match result {
        Ok(_) => {
            println!("Exiting");
            exit(0);
        },
        Err(e) => {
            println!("Error occurred, exiting: {:?}", e);
            exit(1);
        }
    }


}

fn parse_hex_color(hex_color: &str) -> [u8; 3] {
    let z = i64::from_str_radix(hex_color.trim_start_matches('#'), 16).unwrap();
    [
        ((z >> 16) & 0xff) as u8,
        ((z >> 8) & 0xff) as u8,
        (z & 0xff) as u8,
    ]
}
