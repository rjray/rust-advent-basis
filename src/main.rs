// Mostly "borrowed" from https://github.com/nickyvanurk/advent-of-code-rust-template

// Standard libraries used:
use std::env;
use std::fs;
use std::time::{Duration, Instant};

use advent_of_code::{get_day, noop};

fn fmt_time(ms: f64) -> String {
    if ms <= 1.0 {
        let micro_sec = ms * 1000.0;
        return String::from(format!("{}µs", micro_sec.round()));
    }

    if ms < 1000.0 {
        let whole_ms = ms.floor();
        let rem_ms = ms - whole_ms;
        return String::from(format!("{}ms ", whole_ms) + &fmt_time(rem_ms));
    }

    let sec: f64 = ms / 1000.0;
    if sec < 60.0 {
        let whole_sec = sec.floor();
        let rem_ms = ms - whole_sec * 1000.0;

        return format!("{}s ", whole_sec) + &fmt_time(rem_ms);
    }

    let min: f64 = sec / 60.0;
    return format!("{}m ", min.floor()) + &fmt_time((sec % 60.0) * 1000.0);
}

fn fmt_dur(dur: Duration) -> String {
    return fmt_time(dur.as_secs_f64() * 1000.0);
}

fn main() {
    // Start with getting a copy of the args:
    let args: Vec<String> = env::args().collect();
    let mut day: String;
    let data: String;

    if args.len() >= 2 {
        day = args[1].clone();
    } else {
        panic!("Wrong number of arguments");
    }

    // Parse day as a number
    day = day.trim().to_string();
    let day_num: u32 = match day.parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("Invalid day number: {}", day);
        }
    };

    // Now get the input file name
    if args.len() >= 3 {
        data = args[2].clone();
    } else {
        data = format!("day{:02}.txt", day_num);
    }
    let cwd = env::current_dir().unwrap();
    let fname = cwd.join("input").join(data);

    let input = fs::read_to_string(fname).expect("Error while reading");

    let runners = get_day(day_num);

    if runners.0 != noop {
        println!("Running Part 1");
        let start = Instant::now();
        runners.0(input.clone());
        let dur = start.elapsed();
        println!("Time: {}", fmt_dur(dur));
    }
    if runners.1 != noop {
        println!("Running Part 2");
        let start = Instant::now();
        runners.1(input.clone());
        let dur = start.elapsed();
        println!("Time: {}", fmt_dur(dur));
    }
}
