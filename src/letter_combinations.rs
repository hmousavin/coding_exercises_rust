use std::iter::iter;

struct Digit(u8, [char; 4]);
const KEYS: [Digit; 8] = [
    Digit(2, ['a', 'b', 'c', ' ']),
    Digit(3, ['d', 'e', 'f', ' ']),
    Digit(4, ['g', 'g', 'i', ' ']),
    Digit(5, ['j', 'k', 'l', ' ']),
    Digit(6, ['m', 'n', 'o', ' ']),
    Digit(7, ['p', 'q', 'r', 's']),
    Digit(8, ['t', 'u', 'v', ' ']),
    Digit(9, ['w', 'x', 'y', 'z'])
];

// impl Solution {
    pub fn letter_combinations(digits: &str) -> Vec<String> {
        iproduct()
        
        for d in digits.chars() {

        }
        
        KEYS[]
        
        
        // vec!["1".to_string(), "2".to_string()]
    }
// }

fn main() {
    assert_eq!(letter_combinations("23"), vec!["ad","ae","af","bd","be","bf","cd","ce","cf"]);
    // assert_eq!(letter_combinations(""), vec![]);
    assert_eq!(letter_combinations("2"), vec!["a","b","c"]);
}