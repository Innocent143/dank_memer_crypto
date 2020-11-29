mod deck;
use sha2::{Sha256, Digest};

fn main() {
  // Keeping this a secret is integral to making the game unpredictable.
  let secret: Vec<u8> = "hello".as_bytes().to_vec();
  let mut hashes: Vec<Vec<u8>> = Vec::new();

  // Generate 100 hashes - allows for 100 games. In production you should probably generate a billion
  // To prove to users that this is not generated on the fly - a game hash can be used to work out the hash of the last game.
  for i in 0..100 {
    let mut hasher = Sha256::new();
    
    if i == 0 {
        hasher.update(&secret);
    } else {
        hasher.update(&hashes.last().unwrap());
    }

    hashes.push(hasher.finalize().to_vec());
  }

  // Lets iterate through the hashes and run each game
  while let Some(hash) = hashes.pop() {
    println!("Running game {}", hex::encode(&hash));
    let deck = deck::Deck::from_hash(hash);

    println!("The first card in the pack is {:?}", deck.deck.first().unwrap());
  }
}