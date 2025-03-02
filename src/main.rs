// Chapter 1
pub mod hello_cargo;
pub mod hello_world;

// Chapter 2
pub mod guessing_game;

// Chapter 3
pub mod branches;
pub mod comments;
pub mod functions;
pub mod loops;
pub mod no_type_annotations;
pub mod variables;

// Chapter 4

mod ownership;

fn main() {
    // println!("Hello, world!");

    // hello_world::main(); // Chapter 1.2
    // hello_cargo::main(); // Chapter 1.3

    // guessing_game::main();   // Chapter 2

    // variables::main();           // Chapter 3.1
    // no_type_annotations::main(); // Chapter 3.2
    // functions::main();           // Chapter 3.3
    // comments::main();            // Chapter 3.4
    // branches::main();            // Chapter 3.5
    // loops::main();               // Chapter 3.5

    ownership::main(); // Chapter 4.1
}
