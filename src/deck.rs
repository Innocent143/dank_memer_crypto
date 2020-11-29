use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use rand::{SeedableRng, rngs::{StdRng}, RngCore};
use std::convert::TryInto;

#[derive(EnumIter, Debug, Clone)]
pub enum Suit {
  Hearts,
  Diamonds,
  Clubs,
  Spades
}

#[derive(EnumIter, Debug, Clone)]
pub enum Rank {
  Two = 2,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Jack,
  Queen,
  King,
  Ace
}

#[derive(Debug)]
pub struct Card {
  rank: Rank,
  suit: Suit,
  rand: u64
}

#[derive(Debug)]
pub struct Deck {
  pub deck: Vec<Card>
}

impl Deck {
  pub fn from_hash(hash: Vec<u8>) -> Self {
    let a: [u8; 32] = hash[..].try_into().unwrap();
    let mut rng: StdRng = SeedableRng::from_seed(a);

    let mut deck = Vec::new();

    for suit in Suit::iter() {
      for rank in Rank::iter() {
        // Generate a random number for each card
        deck.push(Card {
          rank,
          suit: suit.clone(),
          rand: rng.next_u64()
        });
      }
    }

    // Maybe spice this up a little
    deck.sort_by(|a ,b| a.rand.cmp(&b.rand));

    Deck { deck }
  }
}