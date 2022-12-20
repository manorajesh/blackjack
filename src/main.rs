use rand::thread_rng;
use rand::seq::SliceRandom;
use pancurses::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // let locale_conf = LcCategory::all;
    // setlocale(locale_conf, "en_US.UTF-8");

    let mut player_hand: Vec<String>;
    let mut dealer_hand: Vec<String>;
    let bet = 10;
    let mut total = 1000;
    
    // Clunky city
    let mut deck: Vec<String> = vec!["2♣", "3♣", "4♣", "5♣", "6♣", "7♣", "8♣", "9♣", "10♣", "J♣", 
    "Q♣", "K♣", "A♣","2♥", "3♥", "4♥", "5♥", "6♥", "7♥", "8♥", "9♥", "10♥", "J♥", "Q♥", "K♥", "A♥",
    "2♠", "3♠", "4♠", "5♠", "6♠", "7♠", "8♠", "9♠", "10♠", "J♠", "Q♠", "K♠", "A♠","2♦", "3♦", "4♦", 
    "5♦", "6♦", "7♦", "8♦", "9♦", "10♦", "J♦", "Q♦", "K♦", "A♦"].iter().map(|&x| x.into()).collect();

    let term = initscr();
    noecho();
    curs_set(0);
    start_color();
    use_default_colors();
    init_pair(30, 1, -1);
    let dims = term.get_max_yx();
    let scr = newwin(10, 30, dims.0/2-3, dims.1/2-9);
    scr.keypad(true);

    loop {
        deck.shuffle(&mut thread_rng());
        dealer_hand = deck[0..2].try_into().unwrap();
        player_hand = deck[2..4].try_into().unwrap();

        loop {
            scr.clear();
            scr.attrset(A_DIM);
            scr.addstr("Blackjack────────────\n\n");
            scr.attrset(A_NORMAL);
            scr.addstr(format!("Total: {:3} - Bet: {}\n─────|\n", total, bet));
            scr.addstr(format!("{} *\n", dealer_hand[0]).as_str());
            scr.addstr(player_hand.join(" ").as_str());
            scr.refresh();

            if hand_total(&player_hand) == 21 {
                scr.addstr(format!("\n\nBlackjack! {}", hand_total(&player_hand)).as_str());
                let gain = bet + (bet/2) as i32;
                total += gain;
                scr.mvaddstr(2, 0, format!("Total: {:3} - Bet: {}\n─────| {:+}\n", total, bet, gain));
                scr.refresh();
                scr.getch();
                break;
            }

            match scr.getch() {
                Some(Input::Character(char)) => {
                    if char == 'h' {
                        player_hand.append(&mut draw(deck.clone(), 1));
                        if hand_total(&player_hand) > 21 {
                            scr.clear();
                            scr.attrset(A_DIM);
                            scr.addstr("Blackjack────────────\n\n");
                            scr.attrset(A_NORMAL);
                            scr.addstr(format!("Total: {:3} - Bet: {}\n─────|\n", total, bet));
                            scr.addstr(format!("{} *\n", dealer_hand[0]).as_str());
                            scr.addstr(player_hand.join(" ").as_str());
                            scr.addstr(format!("\n\nLost ;( {}", hand_total(&player_hand)).as_str());
                            let gain = -bet;
                            total += gain;
                            scr.mvaddstr(2, 0, format!("Total: {:3} - Bet: {}\n─────| {:+}\n", total, bet, gain));
                            scr.refresh();
                            scr.getch();
                            break;
                        }
                    } else if char == 's' {
                        break;
                    } else if char == 'q' {
                        endwin();
                        return;
                    }
                },
                _ => ()
            }
        }

        while hand_total(&player_hand) < 21 { // checking for player blackjack
            dealer_hand.append(&mut draw(deck.clone(), 1));
            let dealer_hand_total = hand_total(&dealer_hand);

            scr.clear();
            scr.attrset(A_DIM);
            scr.addstr("Blackjack────────────\n\n");
            scr.attrset(A_NORMAL);
            scr.addstr(format!("Total: {:3} - Bet: {}\n─────|\n", total, bet));
            scr.addstr(format!("{}\n", dealer_hand.join(" ")).as_str());
            scr.addstr(player_hand.join(" ").as_str());
            scr.refresh();
            sleep(Duration::from_millis(500));

            if dealer_hand_total > 21 {
                scr.addstr(format!("\n\nWon! You have {} and dealer has {}", hand_total(&player_hand), hand_total(&dealer_hand)).as_str());
                let gain = bet;
                total += gain;
                scr.mvaddstr(2, 0, format!("Total: {:3} - Bet: {}\n─────| {:+}\n", total, bet, gain));
                scr.refresh();
                scr.getch();
                break;
            } if dealer_hand_total > hand_total(&player_hand) || dealer_hand_total == 21 {
                scr.addstr(format!("\n\nLost ;( You have {} and dealer has {}", hand_total(&player_hand), hand_total(&dealer_hand)).as_str());
                let gain = -bet;
                total += gain;
                scr.mvaddstr(2, 0, format!("Total: {:3} - Bet: {}\n─────| {:+}\n", total, bet, gain));
                scr.refresh();
                scr.getch();
                break;
            }
        }
    }
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