use std::env;
use std::process::Command;

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
                Command::new("sh")
                    .arg("-c")
                    .arg(format!(
                        "networksetup -setairportnetwork {} {} p",
                        args[2], args[3]
                    ))
                    .output()
                    .expect("failed to execute process");
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
