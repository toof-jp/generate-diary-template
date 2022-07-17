use chrono::prelude::*;
use chrono::Duration;

fn generate_diary_template(start_date: &Date<Local>) -> String {
    let mut diary_template = String::new();
    let mut date = Date::clone(start_date);
    diary_template.push_str(&format!("# {}\n", &date.format("%F")));
    for _ in 0..7 {
        diary_template.push_str(&format!("## {}\n\n", &date.format("%F")));
        date = date + Duration::days(1);
    }
    diary_template
}

fn main() {
    let local_date = Local::today();
    println!("{}", generate_diary_template(&local_date));
}
