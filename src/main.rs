use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    println!("--- 몇초로 할까요? setting sec : --- ");
    let mut total_seconds = String::new();
    io::stdin()
        .read_line(&mut total_seconds)
        .expect("Failed to read line");
    let mut seconds_remaining: i32 = total_seconds.trim().parse().expect("InPut not an integer");

    while seconds_remaining > 0 {
        let print_min_remaing = seconds_remaining / 60;
        let print_seconds_remaing = seconds_remaining % 60;

        print!(
            "\r\t\t방송 시작:   {}  분(min) {} 초(sec)   전 ",
            print_min_remaing, print_seconds_remaing
        );
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        seconds_remaining -= 1;
    }

    println!("\r\t\tCountdown finished! 방송 시작 하겠습니다.");
}
