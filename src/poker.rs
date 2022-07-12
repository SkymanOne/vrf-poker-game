extern crate schnorrkel;
use std::io::stdin;

use merlin::Transcript;
use rand::Rng;
use schnorrkel::{
    derive,
    vrf::{VRFInOut, VRFPreOut, VRFProof},
    Keypair, PublicKey,
};

use crate::{recieve, try_draw};

#[derive(Debug)]
struct Player {
    keypair: Keypair,
    cards: Vec<(u16, [u8; 97])>,
    balance: i32,
}

impl Player {
    pub fn new(keypair: Keypair, balance: i32) -> Self {
        Player {
            keypair,
            cards: vec![],
            balance,
        }
    }
    pub fn hand_card(&mut self, cards: Vec<(u16, [u8; 97])>) {
        self.cards = cards;
    }
}

pub fn run() {
    let VRF_seed = &[0u8; 32];

    println!("game starts!");
    let mut input: String = String::new();
    println!("Enter the number of players");
    stdin().read_line(&mut input).expect("error reading string");
    input = input.replace('\n', "");
    let n: i32 = input.parse().unwrap();

    println!("There {} player with $1000 each", n);
    let mut csprng = rand_core::OsRng;
    let mut players: Vec<Player> = (0..n)
        .map(|_| Player::new(Keypair::generate_with(&mut csprng), 1000))
        .collect();

    //each player is given 2 cards
    players.iter_mut().for_each(|player| {
        let cards: Vec<(u16, [u8; 97])> = (0..2)
            .filter_map(|i| try_draw(&player.keypair, VRF_seed, i))
            .collect();
        player.hand_card(cards);
    });

    let mut bank = 0;

    println!("Players are given 2 cards each");
    wait();

    bid(&mut players, &mut bank);

    println!("Bank is {}", bank);
    wait();

    let table = Keypair::generate_with(&mut csprng);
    let mut cards: Vec<(u16, [u8; 97])> = (0..3)
        .filter_map(|i| try_draw(&table, VRF_seed, i))
        .collect();
    println!(
        "Cards on the table are: {:?}",
        reveal_cards(&cards, &table.public, VRF_seed)
    );
    wait();
    bid(&mut players, &mut bank);
    wait();

    //placing 4th card on the table
    let card = try_draw(&table, VRF_seed, 3).unwrap();
    cards.push(card);

    println!(
        "Cards on the table are: {:?}",
        reveal_cards(&cards, &table.public, VRF_seed)
    );
    wait();
    bid(&mut players, &mut bank);
    wait();

    //placing 5th card on the table
    let card = try_draw(&table, VRF_seed, 4).unwrap();
    cards.push(card);

    let table_cards = reveal_cards(&cards, &table.public, VRF_seed);
    println!("Cards on the table are: {:?}", table_cards);
    wait();
    bid(&mut players, &mut bank);
    wait();
    //revealing cards and choosing a winner
    let table_sum: u16 = table_cards.iter().sum();
    let mut highest_score = (0, &PublicKey::default());
    players.iter().for_each(|player| {
        let player_cards = reveal_cards(&player.cards, &player.keypair.public, VRF_seed);
        println!("Player with public key: {:?} has cards: {:?}", player.keypair.public.to_bytes(), player_cards);
        let sum: u16 = player_cards.iter().sum::<u16>();
        let player_sum = table_sum + sum;
        if highest_score.0 < player_sum {
            highest_score = (player_sum, &player.keypair.public);
        }
    });
    println!(
        "Player with public key: {:?} is a winner with the score {}. He wins ${}",
        highest_score.1.to_bytes(),
        highest_score.0,
        bank
    );
}

fn bid(players: &mut [Player], bank: &mut i32) {
    players.iter_mut().for_each(|player| {
        let bid = rand::thread_rng().gen_range(0..301);
        player.balance -= bid;
        println!(
            "Player with key {:?} made a bid of {}",
            player.keypair.public.to_bytes(), bid
        );
        *bank += bid;
    });
}

fn wait() {
    println!("Press enter to continue...");
    stdin()
        .read_line(&mut String::new())
        .expect("error reading line");
}

fn reveal_cards(cards: &[(u16, [u8; 97])], key: &PublicKey, seed: &[u8; 32]) -> Vec<u16> {
    cards
        .iter()
        .filter_map(|card| recieve(key, &card.1, seed))
        .collect()
}
