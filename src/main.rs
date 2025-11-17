use std::time::Duration;
use std::process::Command;
use rand::Rng;
use std::thread;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut seconds = 240;
    if args.len() < 0 {
        seconds = args[0].parse().unwrap();
    }

    Command::new("sh")
        .arg("-c")
        .arg(format!("sudo -i"))
        .output()
        .expect("failed to execute process");

    loop {
        let mut rng = rand::rng();
        let mut new_hostname = String::new();
        for _ in 0..rng.random_range(1..=50) {
            let rand_char = rng.random_range(32..126) as u8 as char;
            new_hostname = format!("{}{}", new_hostname.clone(), rand_char.clone());
        }

        Command::new("sh")
            .arg("-c")
            .arg(format!("hostname {}", &new_hostname.clone()))
            .output()
            .expect("failed to execute process");

        println!("New hostname: {}", new_hostname);
        thread::sleep(Duration::from_secs(seconds));
    }
}
