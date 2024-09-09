use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Rgb {
    red: u8,
    green: u8,
    blue: u8
}

trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    fn r(&self) -> u8 {
        self.red
    }

    fn g(&self) -> u8 {
        self.green
    }

    fn b(&self) -> u8 {
        self.blue
    }
}

impl Rgb {
    fn new(red: u8, green: u8, blue: u8) -> Self {
        Rgb { red, green, blue }
    }
}

impl FromStr for Rgb {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex_str = &s[1..]; // Skip the leading '#'
        if hex_str.len() != 6 {
            return Err(());
        }

        let red = u8::from_str_radix(&hex_str[0..2], 16).map_err(|_| ())?;
        let green = u8::from_str_radix(&hex_str[2..4], 16).map_err(|_| ())?;
        let blue = u8::from_str_radix(&hex_str[4..6], 16).map_err(|_| ())?;

        Ok(Rgb::new(red, green, blue))
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

fn main() {
    let white = Rgb::new(255, 255, 255);
    println!("White: {}", white);
}

#[test]
fn every_color() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let color: Rgb = hex.parse().unwrap();
        assert_eq!(hex, format!("{}", color));
    }
}

#[test]
#[should_panic]
fn too_short () {
    let _: Rgb = "1234".parse().unwrap();
}

#[test]
#[should_panic]
fn not_a_hex_code () {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn invalid_literals () {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn no_leading_hash() {
    let _: Rgb = "aabbcc".parse().unwrap();
}

#[test]
#[should_panic]
fn out_of_bounds() {
    let _: Rgb = "00gg00".parse().unwrap();
}