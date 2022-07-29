use std::fmt::Write as _;
use time::format_description;
use time::Date;

pub fn generate_diary_template(start_date: &Date) -> String {
    let mut diary_template = String::new();
    let mut date = Date::clone(start_date);

    let format = format_description::parse("[year]-[month]-[day]").unwrap();
    writeln!(&mut diary_template, "# {}", &date.format(&format).unwrap()).unwrap();
    for _ in 0..7 {
        writeln!(
            &mut diary_template,
            "## {}\n",
            &date.format(&format).unwrap()
        )
        .unwrap();
        date = date.next_day().unwrap();
    }
    diary_template
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::Month;

    #[test]
    fn test() {
        let date = Date::from_calendar_date(2018, Month::January, 1).unwrap();
        assert_eq!(
            generate_diary_template(&date),
            r"# 2018-01-01
## 2018-01-01

## 2018-01-02

## 2018-01-03

## 2018-01-04

## 2018-01-05

## 2018-01-06

## 2018-01-07

"
        );
    }
}
