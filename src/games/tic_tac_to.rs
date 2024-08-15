
fn print_map(map: Vec<Vec<char>>){

    for i in 0..map.len(){
        for j in 0..map[i].len(){
            print!("{}", map[i][j]);
        }
        print!("\n");
    }
}

fn win_chack(map: Vec<Vec<char>> ) -> bool{
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
                return true;
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
                return true;
            }
            //reset valuse
            streak = 0;

            //thack vurtekole lines
            if map[0 * 2 + 2][0 * 2 + 2] == player && map[1 * 2 + 2][1 * 2 + 2] == player && map[2 * 2 + 2][2 * 2 + 2] == player {
                return true;
            }
            if map[0 * 2 + 2][2 * 2 + 2] == player && map[1 * 2 + 2][1 * 2 + 2] == player && map[2 * 2 + 2][0 * 2 + 2] == player {
                return true;
            }
            //reset valuse
            streak = 0;

            player = 'o';
        }
        player = 'x';
    }
    return false;
}


pub fn start(){
    // to only exes the game tiles use this i*2+3, i = the index 0..2
    let map: Vec<Vec<char>> = vec![vec![' ', '|', '1', '|', '2', '|', '3', '|', ' '],
                                   vec!['—', '+', '—', '—', '—', '—', '—', '+', '—'],
                                   vec!['A', '|', 'o', '|', ' ', '|', 'x', '|', 'A'],
                                   vec!['—', '|', '—', '+', '—', '+', '—', '|', '—'],
                                   vec!['B', '|', ' ', '|', 'o', '|', ' ', '|', 'B'],
                                   vec!['—', '|', '—', '+', '—', '+', '—', '|', '—'],
                                   vec!['C', '|', 'o', '|', ' ', '|', 'o', '|', 'C'],
                                   vec!['—', '+', '—', '—', '—', '—', '—', '+', '—'],
                                   vec![' ', '|', '1', '|', '2', '|', '3', '|', ' ']];


    let player1 = 'x';
    let player2 = 'o';
    let turn = 1;
    let runing = true;

}