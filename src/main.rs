use core::{panic, str};

#[derive(Clone, Copy)]
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

fn parse_input_part2(filepath: &str) -> Vec<(Hand, Hand)> {
    std::fs::read_to_string(filepath)
        .unwrap()
        .split("\n")
        .map(|line| {
            let split = line.split(" ").collect::<Vec<&str>>();

            let oponent = str_to_hand(split[0]);
            let me = determine_hand(&oponent, split[1]);

            (oponent, me)
        })
        .collect::<Vec<(Hand, Hand)>>()
}

fn determine_hand(oponent: &Hand, me: &str) -> Hand {
    match me {
        "X" => match oponent {
            Hand::Rock => Hand::Scissor,
            Hand::Paper => Hand::Rock,
            Hand::Scissor => Hand::Paper
        },

        "Y" => oponent.clone(),

        "Z" => match oponent {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissor,
            Hand::Scissor => Hand::Rock
        },

        _ => panic!("really?...")
    }
}

fn play_match(oponent: &Hand, me: &Hand) -> i32 {
    match me {
        Hand::Rock => match oponent {
            Hand::Paper   => 0,
            Hand::Rock    => 3,
            Hand::Scissor => 6
        },

        Hand::Paper => match oponent {
            Hand::Scissor => 0,
            Hand::Paper   => 3,
            Hand::Rock    => 6
        },

        Hand::Scissor => match oponent {
            Hand::Rock    => 0,
            Hand::Scissor => 3,
            Hand::Paper   => 6
        }
    }
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

    let part2_games = parse_input_part2("input.txt");
    println!("{:}", solve(part2_games));
}
