use core::time;
use games::usfol::user_input;
use std::thread;
use std::time::Duration;

mod games;

fn main() {
    let main_manu_games = vec!["tic tac to", "gasing game"];

    while true {
        println!("enter q to qwit.");
        for i in 0..main_manu_games.len() {
            println!("enter {} to play {}.", i, main_manu_games[i])
        }

        let main_manu_input = &user_input("")[..];
        match main_manu_input {
            "0" => games::tic_tac_to::start(),
            "1" => games::gasing::start(),
            "q" => break,
            &_ => println!("not falid game!"),
        }
        thread::sleep(time::Duration::from_secs(4));
    }
}
