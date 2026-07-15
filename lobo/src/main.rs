#[derive(Debug)]
enum CardSuit {
    A,B,C,D,E
}

#[derive(Debug)]
struct Card {
    suit: CardSuit,
    value: usize,
}

#[derive(Debug)]
struct Deck {
    cards: [Card; 50]
}

fn new_deck() -> Deck {
    let mut new_cards: [Card; 50];

    for i in 1..11 {
        new_cards[(i-1)*5] = Card {
            suit: CardSuit::A,
            value: i,
        };
        new_cards[(i-1)*5+1] = Card {
            suit: CardSuit::B,
            value: i,
        };
        new_cards[(i-1)*5+2] = Card {
            suit: CardSuit::C,
            value: i,
        };
        new_cards[(i-1)*5+3] = Card {
            suit: CardSuit::D,
            value: i,
        };
        new_cards[(i-1)*5+4] = Card {
            suit: CardSuit::E,
            value: i,
        };
    }

    print!("{:?}",new_cards);

    Deck {
        cards: new_cards
    }
}

fn main() {
    println!("Hello, world!");
}
