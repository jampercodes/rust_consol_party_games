extern crate rand;
use rand::Rng;
use super::usfol::user_input;

pub fn start() {
    let mut rng = rand::thread_rng();
    let mut runing = true;
    let mut correct_input = false;
    let mut guessing = 0;
    let mut random_num = 0;
    let mut max = 10;
    let mut player_input = String::new();
    let mut player_ges = 0;

    println!("cose divecolty: ");
    println!("q - qwit");
    println!("1 - esy       - 1..10");
    println!("2 - normol    - 1..100");
    println!("3 - hard      - 1..1.000");
    println!("4 - imposebol - 1..100.000");

    player_input = user_input("1, 2, 3 or 4 >>");

    while !correct_input{
        match &player_input[..]{
            "q" =>  {return; runing = false},
            "1" => {random_num = rng.gen_range(1..10); max = 10; correct_input = true},
            "2" => {random_num = rng.gen_range(1..100); max = 100; correct_input = true},
            "3" => {random_num = rng.gen_range(1..1000); max = 1000; correct_input = true},
            "4" => {random_num = rng.gen_range(1..100000); max = 100000; correct_input = true},
            _=> println!("not a difecolty"),
        }
    }

    while runing {
        print!("plase a  number between 1 and  {} >>", max);
        player_ges = user_input("").parse::<i32>().unwrap();

        if player_ges < random_num{
            println!("the num is BIGER tan {}", player_ges);
            guessing +=1;
        } else if player_ges > random_num{
            println!("the num is SMALER tan {}", player_ges);
            guessing +=1;
        }else if player_ges == random_num{
            println!("you win in {} turns, the number was {}.", guessing, random_num);
            runing = false;
        }
    }

}