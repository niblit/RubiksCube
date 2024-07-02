use rubik_solver::prelude::*;

const SCRAMBLE: &str = "L2 D2 R U2 F2 L U2 L D2 R2 F2 R' D F D R D L' B L'";

fn main() {
    let mut cube = Cube::default();
    println!("Initial position:\n{cube}");

    cube.scramble(SCRAMBLE);
    println!("Scramble: {SCRAMBLE}\n{cube}");
    cube.clear_past_moves();

    println!("Searching white cross...");
    let solved_cube = solve(&cube);
    solved_cube.print_moves();
    println!("{solved_cube}");
}
