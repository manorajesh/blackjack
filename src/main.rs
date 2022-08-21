use rand::thread_rng;
use rand::seq::SliceRandom;
use ncurses::*;

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
    let mut player_hand: Vec<&str> = Vec::new();
    let mut dealer_hand: Vec<&str> = Vec::new();
    let mut bet = String::new();
    let mut total = 1000;
    
    let mut deck = vec!["2c", "3c", "4c", "5c", "6c", "7c", "8c", "9c", "10c", "Jc", "Qc", "Kc", "Ac","2h", "3h", "4h", "5h", "6h", "7h", "8h", "9h", "10h", "Jh", "Qh", "Kh", "Ah","2s", "3s", "4s", "5s", "6s", "7s", "8s", "9s", "10s", "Js", "Qs", "Ks", "As","2d", "3d", "4d", "5d", "6d", "7d", "8d", "9d", "1sd", "Jd", "Qd", "Kd", "Ad"];
    let mut used:Vec<&str> = Vec::new();
    deck.shuffle(&mut thread_rng());

    initscr();
    input();
    noecho();

    let mut usr_input: char = 'a'; // placeholder
    while usr_input != 'q' {
        addstr(&deck[..4].join(" "));
        usr_input = getch() as u8 as char;        
        refresh();
    }
}
