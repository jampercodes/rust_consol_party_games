use super::usfol::user_input;


fn print_map(map: Vec<Vec<char>>){

    for i in 0..map.len(){
        for j in 0..map[i].len(){
            print!("{}", map[i][j]);
        }
        print!("\n");
    }
}

fn win_chack(map: Vec<Vec<char>> ) -> (bool, char){
    let mut streak = 0;
    let mut player = 'x';
    for i in 0..3{
        for j in 0..2{
            //thack horesontol lines
            for f in 0..3{
                if map[i * 2 + 2 as usize][f * 2 + 2 as usize] == player && streak != -1{
                    streak +=1;
                } else {
                    streak = -1;
                }
            } //return true if won
            if streak == 3{
                return (true, player);
            }
            //reset valuse
            streak = 0;
            //thack vurtekole lines
            for f in 0..3{
                if map[f * 2 + 2 as usize][i * 2 + 2 as usize] == player && streak != -1{
                    streak +=1;
                } else {
                    streak = -1;
                }
            } //return true if won
            if streak == 3{
                return (true, player);
            }
            
            //thack vurtekole lines
            if map[0 * 2 + 2][0 * 2 + 2] == player && map[1 * 2 + 2][1 * 2 + 2] == player && map[2 * 2 + 2][2 * 2 + 2] == player {
                return (true, player);
            }
            if map[0 * 2 + 2][2 * 2 + 2] == player && map[1 * 2 + 2][1 * 2 + 2] == player && map[2 * 2 + 2][0 * 2 + 2] == player {
                return (true, player);
            }
            //reset valuse
            streak = 0;

            player = 'o';
        }
        player = 'x';
    }
    return (false, ' ');
}

fn player_move(mut map: Vec<Vec<char>>, input: String, player: char) -> Vec<Vec<char>> {
    let split_input: Vec<char> = input.chars().collect();
    let row_charr = split_input[0];
    let colum_charr = split_input[1];

    let row = match row_charr {
        'A'|'a' => 2,
        'B'|'b' => 4,
        'C'|'c' => 6,
        _ => {println!("Not a valid char"); return map;}
    };

    let col = match colum_charr {
        '1' => 2,
        '2' => 4,
        '3' => 6,
        _ => {println!("Not a valid number"); return map;}
    };

    if map[row][col] == ' ' {  // Ensure the spot is empty
        map[row][col] = player;
    } else {
        println!("Spot already taken!");
    }
    return map;
}

pub fn start(){
    // to only exes the game tiles use this i*2+3, i = the index 0..2
    let mut map: Vec<Vec<char>> = vec![vec![' ', '|', '1', '|', '2', '|', '3', '|', ' '],
                                   vec!['—', '+', '—', '—', '—', '—', '—', '+', '—'],
                                   vec!['A', '|', ' ', '|', ' ', '|', ' ', '|', 'A'],
                                   vec!['—', '|', '—', '+', '—', '+', '—', '|', '—'],
                                   vec!['B', '|', ' ', '|', ' ', '|', ' ', '|', 'B'],
                                   vec!['—', '|', '—', '+', '—', '+', '—', '|', '—'],
                                   vec!['C', '|', ' ', '|', ' ', '|', ' ', '|', 'C'],
                                   vec!['—', '+', '—', '—', '—', '—', '—', '+', '—'],
                                   vec![' ', '|', '1', '|', '2', '|', '3', '|', ' ']];


    let player1 = 'x';
    let player2 = 'o';
    let mut turn = 1;
    let mut runing = true;

    while runing {
        print_map(map.clone());
        if turn == 1{
            map = player_move(map, user_input(""), player1);
            turn +=1;
        } else{
            map = player_move(map.clone(), user_input(""), player2);
            turn = 1;
        }
        let (is_doune, winer) = win_chack(map.clone());
        if is_doune {
            runing = false;
            println!("player whit the charector {}  hase wone", winer);
        }
    }

}