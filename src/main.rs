use crate::scenes::level1::level1;

mod actors;
mod ui;
mod world;
mod scenes;

fn startup() {
    println!("Starting Game!\n");
}

fn gameover() {
    println!("Game Over!");
}

fn main() {
    startup();
    
    // start world and non-actors    
    level1();
    
    gameover();
}
