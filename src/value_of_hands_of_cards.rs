enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}

struct Hand {
    cards: Vec<Card>
}

impl Hand {
    fn new() -> Self {
        Hand {
            cards: vec![]
        }
    }

    fn add(&mut self, card: Card) -> &mut Self {
        self.cards.push(card);
        self
    }

    fn value(&self) -> usize {
        use Card::*;
        let mut total = 0;
        
        for card in &self.cards {
            let value = match card {
                Ace => if total < 10 {11} else {1},
                Two => 2,
                Three => 3,
                Four => 4,
                Five => 5,
                Six => 6,
                Seven => 7,
                Eight => 8,
                Nine => 9,
                Jack | Queen | King => 10,
            };

            total += value
        }

        total
    }

    fn is_losing_hand(&self) -> bool {
        self.value() > 21
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let card_str = match *self {
            Card::Ace => "Ace",
            Card::Two => "2",
            Card::Three => "3",
            Card::Four => "4",
            Card::Five => "5",
            Card::Six => "6",
            Card::Seven => "7",
            Card::Eight => "8",
            Card::Nine => "9",
            Card::Jack => "Jack",
            Card::Queen => "Queen",
            Card::King => "King",
        };
        write!(f, "{}", card_str)
    }
}

fn print_the_cards(hand: &Hand) {
    for card in &hand.cards {
        print!("{card} \t");
    }
    println!();
}

fn main() {
    let mut hand = Hand::new();
    hand.add(Card::King).add(Card::Ace);
    print_the_cards(&hand);
}

#[test]
fn single_ace_counting_as_one() {
    let mut hand = Hand::new();
    hand.add(Card::Eight).add(Card::Nine).add(Card::Ace);
    assert_eq!(hand.value(), 18); // 8 + 9 + Ace = 18
    assert_eq!(hand.is_losing_hand(), false);
}

#[test]
fn two_aces_just_one_of_them_considered_eleven() {
    let mut hand = Hand::new();
    hand.add(Card::Ace).add(Card::Ace).add(Card::Seven);
    assert_eq!(hand.value(), 19); // 11 (Ace) + 1 (Ace) + 7 = 19
    assert_eq!(hand.is_losing_hand(), false);
}

#[test]
fn one_ace_and_one_queen() {
    let mut hand = Hand::new();
    hand.add(Card::Ace).add(Card::Queen);
    assert_eq!(hand.value(), 21); // 11 (Ace) + 10 (Queen) = 21 (Blackjack)
    assert_eq!(hand.is_losing_hand(), false);
}

#[test]
fn three_cards_jack_seven_five() {
    let mut hand = Hand::new();
    hand.add(Card::Jack).add(Card::Seven).add(Card::Five);
    assert_eq!(hand.value(), 22); // 10 + 7 + 5 = 22 (Bust)
    assert_eq!(hand.is_losing_hand(), true);
}

#[test]
fn two_aces_and_single_nine() {
    let mut hand = Hand::new();
    hand.add(Card::Ace).add(Card::Ace).add(Card::Nine);
    assert_eq!(hand.value(), 21); // 11 (First Ace) + 1 (Second Ace) + 9 = 21
    assert_eq!(hand.is_losing_hand(), false);
}