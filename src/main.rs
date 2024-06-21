use rubik_solver::Cube;

fn main() {
    let mut cube = Cube::default();
    println!("{cube}");

    // Each move
    cube.left();
    cube.up();
    println!("{cube}");

    // Scramble
    cube.front();
    cube.up();
    cube.down_prime();
    cube.back();
    cube.up_2();
    cube.right_2();
    cube.down_2();
    cube.right();
    cube.front_2();
    cube.up_2();
    cube.back();
    cube.right_2();
    cube.back_2();
    cube.left_2();
    cube.up_2();
    cube.back();
    cube.down_2();
    cube.front_2();
    cube.down_2();
    cube.right();
    cube.up_prime();
    println!("{cube}");
}
