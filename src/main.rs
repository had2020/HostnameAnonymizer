use std::process;

#[derive(PartialEq, Debug)]
enum OS {
    Windows, // Can't help you G.
    Macos,   // My Target
    Linux,   // WIP
    Unknown,
}

fn main() {
    #[cfg(target_os = "windows")]
    let os = OS::Windows;
    #[cfg(target_os = "macos")]
    let os = OS::Macos;
    #[cfg(target_os = "linux")]
    let os = OS::Linux;
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    let os = OS::Unknown;

    if os == OS::Macos || os == OS::Linux {
        println!("Using OS settings for {:?}", os);
    } else {
        println!("Running on an unknown or windows OS. Can't find proper OS commands.");
        process::exit(1);
    }

    println!("Changing LocalHostName name");
}
