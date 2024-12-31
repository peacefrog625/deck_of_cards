pub struct Deck {
    cards: [Card; 52],
}
#[derive(Debug)]
pub struct Card {
    rank: u8,
    suit: Suit,
}
#[derive(Debug, Copy, Clone)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}
impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::<Card>::new();
        let suits = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
        for suit in suits.iter() {
            for rank in 1..=13 {
                cards.push(Card { rank, suit: *suit });
            }
        }
        Deck {
            cards: cards.try_into().expect("Failed to create a new deck"),
        }
    }
    pub fn print(&self) {
        for card in self.cards.iter() {
            match card.rank {
                1 => println!("Ace of {:?}", card.suit),
                11 => println!("Jack of {:?}", card.suit),
                12 => println!("Queen of {:?}", card.suit),
                13 => println!("King of {:?}\n", card.suit),
                _ => println!("{:?} of {:?}", card.rank,card.suit)
            }
        }
    }
}

fn main() {
    let deck = Deck::new();
    deck.print();
}
