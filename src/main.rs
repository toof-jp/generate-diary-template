use anyhow::Result;
use diary_template_generator::*;
use time::OffsetDateTime;

fn main() -> Result<()> {
    let local_date = OffsetDateTime::now_local()?.date();
    let first_day_of_week = get_first_day_of_week(&local_date)?;
    print!("{}", generate_diary_template(&first_day_of_week)?);

    Ok(())
}
