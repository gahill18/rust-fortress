use crate::scenes::*;

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
        
    tutorial::tutorial();
    level1::level1();
    
    gameover();
}
