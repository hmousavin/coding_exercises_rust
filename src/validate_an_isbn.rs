use std::str::FromStr;

struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

#[derive(Debug)]
enum InvalidIsbn {
    TooLong,
    TooShort,
    FailedChecksum,
    InvalidCharacter(usize, char)
}

impl FromStr for Isbn {
    type Err = InvalidIsbn;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw = s.trim().replace("-", "");

        if raw.len() != 13 {
            if raw.len() > 13 {
                return Err(InvalidIsbn::TooLong);
            } else {
                return Err(InvalidIsbn::TooShort);
            }
        }

        let digits: Vec<u8> = raw.chars().enumerate().map(|(i, c)| {
            c.to_digit(10).map(|d| d as u8).ok_or_else(|| InvalidIsbn::InvalidCharacter(i, c))
        }).collect::<Result<_, _>>()?;

        let check_digit = digits[12];
        let calculated_check_digit = calculate_check_digit(&digits[..12]);
        
        if check_digit != calculated_check_digit {
            return Err(InvalidIsbn::FailedChecksum);
        }

        Ok(Isbn { raw: s.to_string(), digits })
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    let sum: u8 = digits.iter()
        .enumerate()
        .map(|(i, &d)| if i % 2 == 0 { d } else { d * 3 })
        .sum();

    let remainder = sum % 10;
    let check_digit = 10 - remainder;
    check_digit % 10
}

fn main() {
    let rust_in_action: Isbn = "978-3-16-148410-0".parse().unwrap();

    println!("Rust in Action's ISBN-13 ({})is valid!", rust_in_action);
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let isbn: Isbn = "978-3-16-148410-0".parse().unwrap();
    assert_eq!(isbn.raw, "978-3-16-148410-0");
}