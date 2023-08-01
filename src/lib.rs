use anyhow::{Context, Result};
use std::fmt::Write as _;
use time::format_description;
use time::Date;
use time::Weekday;

pub fn generate_diary_template(date: &Date) -> Result<String> {
    let mut diary_template = String::new();
    let mut date = Date::clone(date);

    while date.weekday() != Weekday::Monday {
        date = date.previous_day().context("")?;
    }

    let format = format_description::parse("[year]-[month]-[day]")?;
    writeln!(&mut diary_template, "# {}", &date.format(&format)?)?;
    for _ in 0..7 {
        writeln!(&mut diary_template, "## {}\n", &date.format(&format)?)?;
        date = date.next_day().context("")?;
    }
    Ok(diary_template)
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::Month;

    #[test]
    fn test() -> Result<()> {
        let date = Date::from_calendar_date(2022, Month::January, 1).unwrap();
        assert_eq!(
            generate_diary_template(&date)?,
            r"# 2021-12-27
## 2021-12-27

## 2021-12-28

## 2021-12-29

## 2021-12-30

## 2021-12-31

## 2022-01-01

## 2022-01-02

"
        );
        Ok(())
    }
}
