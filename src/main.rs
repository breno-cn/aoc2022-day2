use core::{panic, str};
use std::{collections::HashMap};

#[derive(Eq, PartialEq, Hash)]
enum Hand { Rock, Paper, Scissor }

fn str_to_hand(hand: &str) -> Hand {
    match hand {
        "A" => Hand::Rock,
        "B" => Hand::Paper,
        "C" => Hand::Scissor,
        "X" => Hand::Rock,
        "Y" => Hand::Paper,
        "Z" => Hand::Scissor,
        _   => panic!("...")
    }
}

fn parse_input(filepath: &str) -> Vec<(Hand, Hand)> {
    std::fs::read_to_string(filepath)
        .unwrap()
        .split("\n")
        .map(|line| {
            let split = line.split(" ").collect::<Vec<&str>>();
            
            (str_to_hand(split[0]), str_to_hand(split[1]))
        })
        .collect::<Vec<(Hand, Hand)>>()
}

fn play_match(oponent: &Hand, me: &Hand) -> i32 {
    let results = HashMap::from([
        (Hand::Rock, HashMap::from([
            (Hand::Paper, 0),
            (Hand::Rock, 3),
            (Hand::Scissor, 6)  
        ])),

        (Hand::Paper, HashMap::from([
            (Hand::Scissor, 0),
            (Hand::Paper, 3),
            (Hand::Rock, 6)
        ])),

        (Hand::Scissor, HashMap::from([
            (Hand::Rock, 0),
            (Hand::Scissor, 3),
            (Hand::Paper, 6)
        ]))
    ]);

    results[me][oponent]
}

fn hand_score(hand: &Hand) -> i32 {
    match hand {
        Hand::Rock    => 1,
        Hand::Paper   => 2,
        Hand::Scissor => 3
    }
}

fn solve(games: Vec<(Hand, Hand)>) -> i32 {
    games.iter()
        .map(|(oponent, me)| play_match(oponent, me) + hand_score(me))
        .sum()
}

fn main() {
    let games = parse_input("input.txt");

    println!("{:}", solve(games));
}
