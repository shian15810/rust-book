// Chapter 1
pub mod hello_cargo;
pub mod hello_world;

// Chapter 2

mod guessing_game;

fn main() {
    // println!("Hello, world!");

    // hello_world::main(); // Chapter 1.2
    // hello_cargo::main(); // Chapter 1.3

    guessing_game::main(); // Chapter 2
}
