use anyhow::Result;
use diary_template_generator::generate_diary_template;
use time::OffsetDateTime;

fn main() -> Result<()> {
    let local_date = OffsetDateTime::now_local()?.date();
    println!("{}", generate_diary_template(&local_date)?);

    Ok(())
}
