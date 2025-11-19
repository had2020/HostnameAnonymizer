use rand::Rng;
use std::env;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut seconds = 240;
    if args.len() > 1 {
        seconds = args[1]
            .parse()
            .expect(&format!("{} is not a number!", args[1]));
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

        Command::new("sh")
            .arg("-c")
            .arg("sh mac.sh")
            .output()
            .expect("failed to execute process");

        println!("New hostname: {}", new_hostname);
        println!("New Mac Address");
        thread::sleep(Duration::from_secs(seconds));
    }
}
