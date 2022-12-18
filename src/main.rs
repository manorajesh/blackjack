use rand::thread_rng;
use rand::seq::SliceRandom;
use ncursesw::*;
use std::thread::sleep;
use std::time::Duration;

fn input() {
    let mut buffer = String::new();
    let mut usr_input: u8 = 0;

    while usr_input != '\n' as u8 {
        usr_input = getch() as u8;
        buffer.push(usr_input as char);
        refresh();
    }
}

fn main() {
    let mut player_hand: Vec<String>;
    let mut dealer_hand: Vec<String>;
    let mut bet = String::new();
    let mut total = 1000;
    
    // Clunky city
    let mut deck: Vec<String> = vec!["2♣", "3♣", "4♣", "5♣", "6♣", "7♣", "8♣", "9♣", "10♣", "J♣", 
    "Q♣", "K♣", "A♣","2♥", "3♥", "4♥", "5♥", "6♥", "7♥", "8♥", "9♥", "10♥", "J♥", "Q♥", "K♥", "A♥",
    "2♠", "3♠", "4♠", "5♠", "6♠", "7♠", "8♠", "9♠", "10♠", "J♠", "Q♠", "K♠", "A♠","2♦", "3♦", "4♦", 
    "5♦", "6♦", "7♦", "8♦", "9♦", "10♦", "J♦", "Q♦", "K♦", "A♦"].iter().map(|&x| x.into()).collect();

    initscr();
    noecho();

    let mut usr_input: char = 'a'; // placeholder
    while usr_input != 'q' {
        deck.shuffle(&mut thread_rng());
        dealer_hand = deck[0..2].try_into().unwrap();
        player_hand = deck[2..4].try_into().unwrap();

        loop {
            clear();
            addstr(format!("{} *\n", dealer_hand[0]).as_str());
            addstr(player_hand.join(" ").as_str());
            refresh();

            if hand_total(&player_hand) == 21 {
                addstr(format!("\nBlackjack! {}", hand_total(&player_hand)).as_str());
                refresh();
                getch();
                break;
            }
    
            usr_input = getch() as u8 as char;
            if usr_input == 's' {
                break;
            } else if usr_input == 'h' {
                player_hand.append(&mut draw(deck.clone(), 1));
                if hand_total(&player_hand) > 21 {
                    clear();
                    addstr(format!("{} *\n", dealer_hand[0]).as_str());
                    addstr(player_hand.join(" ").as_str());
                    addstr(format!("\nLost ;( {}", hand_total(&player_hand)).as_str());
                    refresh();
                    getch();
                    break;
                }
            }
        }

        while hand_total(&player_hand) < 21 { // checking for player blackjack
            dealer_hand.append(&mut draw(deck.clone(), 1));
            let dealer_hand_total = hand_total(&dealer_hand);

            clear();
            addstr(format!("{}\n", dealer_hand.join(" ")).as_str());
            addstr(player_hand.join(" ").as_str());
            refresh();
            sleep(Duration::from_millis(500));

            if dealer_hand_total > 21 {
                addstr(format!("\n\nWon! You have {} and dealer has {}", hand_total(&player_hand), hand_total(&dealer_hand)).as_str());
                refresh();
                getch();
                break;
            } else if dealer_hand_total > hand_total(&player_hand) || dealer_hand_total == 21 {
                addstr(format!("\n\nLost ;( You have {} and dealer has {}", hand_total(&player_hand), hand_total(&dealer_hand)).as_str());
                refresh();
                getch();
                break;
            }
        }
    }
    endwin();
}

fn draw(mut deck: Vec<String>, num_of_cards: usize) -> Vec<String> {
    let mut drawn: Vec<String> = Vec::new();
    deck.shuffle(&mut thread_rng());

    for _ in 0..num_of_cards {
        let card = deck.pop();
        if let Some(card) = card {
            drawn.push(card.to_string());
        } else {
            panic!("Looks like you found a bug. Report this code: 01")
        }
    }
    drawn
}

fn hand_total(hand: &Vec<String>) -> usize {
    let mut total: u8 = 0;
    let mut num_of_aces = 0;
    for card in hand {
        let card = card.as_bytes();
        if card.contains(&b'0') {
            total += 10;
        } else if card[0].is_ascii_digit() {
            total += card[0] - b'0';
        } else if card[0] != b'A' {
            total += 10;
        } else {
            num_of_aces += 1;
        }
    }

    if num_of_aces > 0 {
        for _ in 0..num_of_aces {
            if (total+11) <= 21 {
                total += 11;
            } else {
                total += 1;
            }
        }
    }
    total as usize
}