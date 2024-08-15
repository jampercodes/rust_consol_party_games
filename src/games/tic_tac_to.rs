
fn print_map(map: Vec<Vec<char>>){

    for i in 0..map.len(){
        for j in 0..map[i].len(){
            print!("{}", map[i][j]);
        }
        print!("\n");
    }
}


pub fn start(){
    // to only exes the game tiles use this i*2+1, i = the index 0..2
    let map: Vec<Vec<char>> = vec![vec![' ', '|', '1', '|', '2', '|', '3', '|', ' '],
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
    let turn = 1;


}