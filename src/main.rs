use chrono::prelude::*;
use chrono::Duration;
use std::fmt::Write as _;

fn generate_diary_template(start_date: &Date<Local>) -> String {
    let mut diary_template = String::new();
    let mut date = Date::clone(start_date);
    write!(&mut diary_template, "# {}\n", &date.format("%F")).unwrap();
    for _ in 0..7 {
        write!(&mut diary_template, "## {}\n\n", &date.format("%F")).unwrap();
        date = date + Duration::days(1);
    }
    diary_template
}

fn main() {
    let local_date = Local::today();
    println!("{}", generate_diary_template(&local_date));
}
