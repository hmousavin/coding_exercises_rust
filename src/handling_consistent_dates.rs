use chrono::NaiveDate;
use regex::Regex;

/* Parses a string that represents a date. When a date is unable to be determined, return None.  */
fn flexible_date_parse(text: &str) -> Option<NaiveDate> {
    let date_component_reg = Regex::new(r"\d+|Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec").unwrap();
    
    let mut date = String::new();
    let mut separator_needed = false;

    for (_, cap) in date_component_reg.captures_iter(text).enumerate() {
        let token = &cap[0]; 
        let token = match token {
            "Jan" => "01",
            "Feb" => "02",
            "Mar" => "03",
            "Apr" => "04",
            "May" => "05",
            "Jun" => "06",
            "Jul" => "07",
            "Aug" => "08",
            "Sep" => "09",
            "Oct" => "10",
            "Nov" => "11",
            "Dec" => "12",
            _ => token, // Keep the digits as they are
        };

        if separator_needed {
            date.push('-');
        }

        date.push_str(token);
        separator_needed = true;
    }

    NaiveDate::parse_from_str(&date, "%Y-%m-%d").ok()
}

fn main() {
    let dates = [
        "2010-12-11",
        "1999/Mar/02",
        "01.Mar.2021",
        "Mar.05.2021",
        "not a date",
    ];

    for d in dates.iter() {
        println!("{} -> {:?}", d, flexible_date_parse(d));
    }
}

#[test]
fn ymd_hyphen() {
    assert_eq!(flexible_date_parse("2010-12-11"), Some(NaiveDate::from_ymd_opt(2010, 12, 11).unwrap()))
}

#[test]
fn ymd_slash() {
    assert_eq!(flexible_date_parse("1999/Mar/02"), Some(NaiveDate::from_ymd_opt(1999, 3, 2).unwrap()))
}

#[test]
fn ymd_dot() {
    assert_eq!(flexible_date_parse("2021.Mar.01"), Some(NaiveDate::from_ymd_opt(2021, 3, 1).unwrap()))
}

#[test]
fn invalid() {
    assert_eq!(flexible_date_parse("not a date"), None)
}
