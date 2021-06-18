use notify_rust::Notification;
use std::env;
use std::process;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage pomodoro work_duration pause_duration");
        process::exit(1);
    }
    let work_duration: u64 = args[1]
        .parse()
        .expect("Work duration is not a valid number");
    let pause_duration: u64 = args[2]
        .parse()
        .expect("Pause duration is not a valid number");

    loop {
        notify("Back to work !");
        thread::sleep(Duration::from_secs(work_duration * 60));
        notify("Time for a break !");
        thread::sleep(Duration::from_secs(pause_duration * 60));
    }
}

fn notify(msg: &str) {
    Notification::new()
        .body(msg)
        .icon("clock")
        .appname("Pomodoro")
        .timeout(0)
        .show()
        .expect("Could not show notification");
}
