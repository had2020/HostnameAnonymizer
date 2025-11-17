use std::time::Duration;
use std::process::Command;
use std::time::Instant;
use rand::Rng;
use std::thread;

fn main() {
    loop {
        let mut rng = rand::rng();
        let mut new_hostname = String::new();
        for _ in 0..rng.random_range(1..=50) {
            let rand_char = rng.random_range(32..126) as u8 as char;
            new_hostname = format!("{}{}", new_hostname.clone(), rand_char.clone());
        }

        Command::new("sh")
            .arg("-c")
            .arg(format!("sudo hostname {}", &new_hostname.clone()))
            .output()
            .expect("failed to execute process");

        println!("New hostname: {}", new_hostname);
        thread::sleep(Duration::from_mins(2));
    }
}
