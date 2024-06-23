use rubik_solver::Cube;

fn main() {
    let mut cube = Cube::default();
    println!("{cube}");

    cube.scramble("L2 D2 R U2 F2 L U2 L D2 R2 F2 R' D F D R D L' B L'");
    println!("{cube}");

}
