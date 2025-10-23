use std::process::Command;
use std::{env, string};

#[derive(PartialEq, Debug)]
pub enum OS {
    Windows, // Can't help you G.
    Macos,   // My Target
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
        println!("Using OS settings for {:?}", os);
        if args.len() == 1 {
            match args[0] {
                format!("") => {}
                _ => {
                    println!(r#"Invaid Arg try "--help"#)
                }
            }
        } else {
            println!(r#"Requires Args, try "--help""#)
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

    println!("Changing LocalHostName name");
}
