use std::env;
use std::process::Command;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::time::Instant;

#[derive(PartialEq, Debug)]
pub enum OS {
    Windows, // Can't help you G.
    Macos,   // My Target interface likely en0
    Linux,   // WIP
    Unknown,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    #[cfg(target_os = "windows")]
    let os = OS::Windows;
    #[cfg(target_os = "macos")]
    let os = OS::Macos;
    #[cfg(target_os = "linux")]
    let os = OS::Linux;
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    let os = OS::Unknown;

    if os == OS::Macos {
        println!("{:?}", args);
        if args.len() > 1 {
            if args[1] == String::from("breacher") {
            } else if args[1] == String::from("help") {
                println!("*Finding your wifi interface*");
                println!(r#"    networksetup -listallhardwareports"#);
                println!("And get your interface on your wifi device for the interface arg.");
                println!(" ");
                println!("Network breach testing:");
                println!("  breach device_interface ssid");
                println!(" ");
                println!("List all commands");
                println!("  --help")
            } else if args[1] == String::from("breach") {
                let cracked = Arc::new(AtomicBool::new(false));
                let c = cracked.clone();

                while !cracked.load(Ordering::Relaxed) {
                    //std::thread::yield_now();
                    let value = args.clone();
                    thread::spawn(move || {
                        println!("1");
                        println!("2");
                        let output = Command::new("sh")
                            .arg("-c")
                            .arg(format!(
                                "networksetup -setairportnetwork {} {} p",
                                value[2], value[3]
                            ))
                            .output()
                            .expect("failed to execute process");

                        if output.status.success() {
                            println!("Connected successfully!");
                        } else {
                            eprintln!(
                                "Failed to connect. Exit code: {:?}\nStderr: {}",
                                output.status.code(),
                                String::from_utf8_lossy(&output.stderr)
                            );
                        }
                        // networksetup -getinfo Wi-Fi
                        //c.store(true, Ordering::Relaxed);
                    });
                }

                println!("Cracked = {}", cracked.load(Ordering::Relaxed));

                /*
                Command::new("sh")
                    .arg("-c")
                    .arg(format!(
                        "networksetup -setairportnetwork {} {} p",
                        args[2], args[3]
                    ))
                    .output()
                    .expect("failed to execute process");
                */
            } else {
                println!(r#"Invaid Arg try "help"#);
            }
        } else {
            println!(r#"Requires Args, try "help""#)
        }
        /*
        Command::new("sh")
            .arg("-c")
            .arg(r#"sudo scutil --set HostName "NewHostName""#)
            .output()
            .expect("failed to execute process");
        */
    } else {
        println!("Running on an unknown or windows OS. Can't find proper OS commands.");
        std::process::exit(1);
    }
}
