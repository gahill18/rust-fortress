use crate::actors::{actor, player, friendly, enemy};
use {actor::*, actor::Faction, player::PC, friendly::FNPC, enemy::ENPC};

pub fn level1() {
    println!("*LEVEL 1 START*\n");

    // start player actors
    let mut player = PC::new("Player 1", 100, 10, 3, Faction::Player);
    // start non player actors
    let mut minion = FNPC::new("Minion 1", 40, 5, 0, Faction::Player);
    let mut enemy = ENPC::new("Enemy 1", 50, 10, 2, Faction::Enemy);

    player.move_up();
    
    println!("\n*LEVEL 1 END*\n");
}
