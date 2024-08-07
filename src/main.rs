use rubik_solver::prelude::*;

const SCRAMBLE: &str = "L2 D2 R U2 F2 L U2 L D2 R2 F2 R' D F D R D L' B L'";

fn main() {
    let mut cube = Cube::default();
    println!("Initial position:\n{cube}");

    println!("Enter a valid scramble:");
    let mut user_scramble = String::new();
    std::io::stdin().read_line(&mut user_scramble).unwrap();
    user_scramble = user_scramble.replace('\n', "");

    println!();
    if !cube.scramble(&user_scramble) {
        println!("Provided scramble was invalid, using the default one");
        cube.scramble(SCRAMBLE);
    }

    println!("Scramble: [{}]\n{}", cube.moves_as_string(), cube);

    println!("Searching white cross...");
    let solution = solve(&cube);
    cube.clear_past_moves();
    for to_move in solution {
        cube.make_move(to_move);
    }
    println!("Found solution: [{}]\n{}", cube.moves_as_string(), cube);
}
