use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

trait MerdiemConv {
    fn to_military(&self) -> String;
}

struct Date<'a> {
    hour: i32,
    minute: i32,
    second: i32,
    meridiem: &'a str,
}

impl<'a> MerdiemConv for Date<'a> {
    fn to_military(&self) -> String {
        let mut military_time = String::new();
        if self.meridiem.to_uppercase() == "PM" {
            // 12:00:00 - 11:59:59
            if self.hour == 12 {
                // 12:00:00 - 12:59:59
                military_time.push_str("12");
            } else {
                // 01:00:00 - 11:59:59
                military_time.push_str(&format!("{:02}", self.hour + 12));
            }
        } else {
            // 12:00:00 - 11:59:59
            if self.hour == 12 {
                // 12:00:00 - 12:59:59
                military_time.push_str("00");
            } else {
                // 1:00:00 - 11:59:59
                military_time.push_str(&format!("{:02}", self.hour));
            }
        }
        military_time.push_str(":");
        military_time.push_str(&format!("{:02}", &self.minute));
        military_time.push_str(":");
        military_time.push_str(&format!("{:02}", &self.second));

        military_time
    }
}

fn timeConversion(s: &str) -> String {
    println!("Converting: {}", s);
    let meridiem = &s[s.len() - 2..];
    let date = &s[..s.len() - 2];
    let parts: Vec<&str> = date.split(':').collect();
    let date = Date {
        hour: parts[0].parse::<i32>().unwrap(),
        minute: parts[1].parse::<i32>().unwrap(),
        second: parts[2].parse::<i32>().unwrap(),
        meridiem: meridiem,
    };
    println!("{:?}", parts);

    date.to_military()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
