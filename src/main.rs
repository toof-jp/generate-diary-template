use diary_template_generator::generate_diary_template;
use time::OffsetDateTime;

fn main() {
    let local_date = OffsetDateTime::now_local().unwrap().date();
    println!("{}", generate_diary_template(&local_date));
}
