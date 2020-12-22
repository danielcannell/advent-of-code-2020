use std::collections::{HashSet, VecDeque};

pub fn solve() {
    let input = include_str!("../input/day22");
    let mut input_parts = input.split("\n\n");
    let player_1_input = input_parts.next().unwrap();
    let player_2_input = input_parts.next().unwrap();

    let player_1: Vec<u32> = player_1_input
        .trim()
        .lines()
        .skip(1)
        .map(|l| l.parse().unwrap())
        .collect();

    let player_2: Vec<u32> = player_2_input
        .trim()
        .lines()
        .skip(1)
        .map(|l| l.parse().unwrap())
        .collect();

    println!("Part 1: {}", part1(&player_1, &player_2));
    println!("Part 2: {}", part2(&player_1, &player_2));
}

fn part1(player_1: &[u32], player_2: &[u32]) -> u32 {
    let mut player_1: VecDeque<u32> = player_1.iter().cloned().collect();
    let mut player_2: VecDeque<u32> = player_2.iter().cloned().collect();

    while !player_1.is_empty() && !player_2.is_empty() {
        let top_1 = player_1.pop_front().unwrap();
        let top_2 = player_2.pop_front().unwrap();

        if top_1 > top_2 {
            player_1.push_back(top_1);
            player_1.push_back(top_2);
        } else {
            player_2.push_back(top_2);
            player_2.push_back(top_1);
        }
    }

    let mut winner = if !player_1.is_empty() {
        player_1
    } else {
        player_2
    };
    let mut score = 0;
    let mut idx = 0;

    while let Some(card) = winner.pop_back() {
        idx += 1;
        score += idx * card;
    }

    score
}

fn part2(player_1: &[u32], player_2: &[u32]) -> u32 {
    let (_, mut deck) = recursive_combat(player_1.iter().copied(), player_2.iter().copied());

    let mut score = 0;
    let mut idx = 0;

    while let Some(card) = deck.pop_back() {
        idx += 1;
        score += idx * card;
    }

    score
}

fn recursive_combat(
    player_1: impl Iterator<Item = u32>,
    player_2: impl Iterator<Item = u32>,
) -> (Player, VecDeque<u32>) {
    let mut previous_states = HashSet::new();
    let mut player_1: VecDeque<u32> = player_1.collect();
    let mut player_2: VecDeque<u32> = player_2.collect();

    while !player_1.is_empty() && !player_2.is_empty() {
        let state = (
            player_1.iter().copied().collect::<Vec<u32>>(),
            player_2.iter().copied().collect::<Vec<u32>>(),
        );

        if previous_states.contains(&state) {
            return (Player::Player1, player_1);
        }

        previous_states.insert(state);

        let top_1 = player_1.pop_front().unwrap();
        let top_2 = player_2.pop_front().unwrap();

        let winner = if player_1.len() as u32 >= top_1 && player_2.len() as u32 >= top_2 {
            // Play a recursive game to determine the winner
            let (winner, _) = recursive_combat(
                player_1.iter().copied().take(top_1 as usize),
                player_2.iter().copied().take(top_2 as usize),
            );
            winner
        } else {
            // Winner is the player with the highest card
            if top_1 > top_2 {
                Player::Player1
            } else {
                Player::Player2
            }
        };

        match winner {
            Player::Player1 => {
                player_1.push_back(top_1);
                player_1.push_back(top_2);
            }
            Player::Player2 => {
                player_2.push_back(top_2);
                player_2.push_back(top_1);
            }
        }
    }

    if !player_1.is_empty() {
        (Player::Player1, player_1)
    } else {
        (Player::Player2, player_2)
    }
}

#[derive(Debug, Clone, Copy)]
enum Player {
    Player1,
    Player2,
}

#[test]
fn part1_example() {
    let player_1 = vec![9, 2, 6, 3, 1];
    let player_2 = vec![5, 8, 4, 7, 10];
    assert_eq!(part1(&player_1, &player_2), 306);
}

#[test]
fn part2_example() {
    let player_1 = vec![9, 2, 6, 3, 1];
    let player_2 = vec![5, 8, 4, 7, 10];
    assert_eq!(part2(&player_1, &player_2), 291);
}
