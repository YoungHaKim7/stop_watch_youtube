use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() {
    let total_seconds = 600;
    let mut seconds_remaining = total_seconds;

    while seconds_remaining > 0 {
        print!("\r\t\t방송 시작 {}  초 전 ", seconds_remaining);
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        seconds_remaining -= 1;
    }

    println!("Countdown finished! 방송 시작 하겠습니다.");
}
