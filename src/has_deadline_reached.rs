use chrono::{Local, NaiveDate};

struct ImportantEvent {
    what: String,
    date: NaiveDate,
}

trait Deadline {
    fn is_passed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        self.date < Local::now().date_naive()
    }
}

fn main() {
    let missed_christmas = ImportantEvent {
        what: String::from("Christmas"),
        date: NaiveDate::from_ymd(2020, 12, 25)
    };

    if missed_christmas.is_passed() {
        println!("oh well, maybe the next year! 😒")
    }
    else {
        println!("🎄")
    }
}

#[test]
fn test_in_past() {
    let past_event = ImportantEvent {
        what: String::from("Old Event"),
        date: NaiveDate::from_ymd(2020, 1, 1)
    };
    assert!(past_event.is_passed(), "Event should be considered passed.");
}

#[test]
fn test_in_future() {
    let future_event = ImportantEvent {
        what: String::from("Future Event"),
        date: NaiveDate::from_ymd(2025, 1, 1)
    };
    assert!(!future_event.is_passed(), "Event should not be considered passed.");
}