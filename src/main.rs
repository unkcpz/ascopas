use tokio::net::TcpListener;

#[derive(Debug)]
enum Card {
    Coppe(u8),
    Ori(u8),
    Spada(u8),
    Bastoni(u8),
}

struct TableState {}

enum PlayerStates {
    HandState {},
    PocketState {},
}

#[tokio::main]
async fn main() {
    todo!()
}

// Capture the table state and return the message
// FIXME: it can have at most four cards at table
// It requires at most three level loops to tell if the cards captured.
fn capture(cards: Vec<&Card>, played_card: &Card) -> String {
    // Extract the value from the played card
    let played_value = match played_card {
        Card::Ori(num) | Card::Coppe(num) | Card::Spada(num) | Card::Bastoni(num) => num,
    };

    // Check for a single card with the same value
    if let Some(single_match) = cards.iter().find(|card| {
        matches!(card, Card::Ori(num) | Card::Coppe(num) | Card::Spada(num) | Card::Bastoni(num) if *num == *played_value)
    }) {
        return format!("Captured single card: {:?}", single_match);
    }

    // Check for two cards whose values sum to the played card's value
    for (i, c1) in cards.iter().enumerate() {
        for c2 in cards.iter().skip(i + 1) {
            let value1 = match c1 {
                Card::Ori(num) | Card::Coppe(num) | Card::Spada(num) | Card::Bastoni(num) => num,
            };
            let value2 = match c2 {
                Card::Ori(num) | Card::Coppe(num) | Card::Spada(num) | Card::Bastoni(num) => num,
            };

            if value1 + value2 == *played_value {
                return format!("Captured two cards: {:?} and {:?}", c1, c2);
            }
        }
    }

    // If no matches found, return no capture
    "No cards captured".to_string()
}
