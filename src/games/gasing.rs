extern crate rand;
use rand::Rng;
use super::usfol::user_input;

pub fn start() {
    let mut rng = rand::thread_rng();
    let mut runing = true;
    let mut correct_input = false;

    println!("cose divecolty: ");
    println!("q - qwit");
    println!("1 - esy       - 1..10");
    println!("2 - normol    - 1..100");
    println!("3 - hard      - 1..1.000");
    println!("4 - imposebol - 1..100.000");

    let mut player_input = user_input("1, 2, 3 or 4 >>");
    let mut random_num = 0;

    while !correct_input{
        match &player_input[..]{
            "q" =>  break,
            "1" => {random_num = rng.gen_range(0..10); correct_input = true},
            "2" => {random_num = rng.gen_range(0..100); correct_input = true},
            "3" => {random_num = rng.gen_range(0..1000); correct_input = true},
            "4" => {random_num = rng.gen_range(0..100000); correct_input = true},
            _=> println!("not a difecolty"),
        }
    }


    println!("random num ={}", random_num);
    while false {

    }

}