use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() {
    let total_seconds = 600;
    let mut seconds_remaining = total_seconds;

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

    println!("Countdown finished! 방송 시작 하겠습니다.");
}
