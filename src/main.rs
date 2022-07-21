use generate_diary_template::generate_diary_template;

use chrono::Local;

fn main() {
    let local_date = Local::today();
    println!("{}", generate_diary_template(&local_date));
}
