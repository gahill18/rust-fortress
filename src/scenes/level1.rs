use crate::actors::{actor, player, friendly, enemy};
use {actor::Actor, actor::Faction, player::PC, friendly::FNPC, enemy::ENPC};

pub fn level1() {
    // start player actors
    let mut player = PC::new("Player 1", 100, 10, 3, Faction::Player);
    // start non player actors
    let mut minion = FNPC::new("Minion 1", 40, 5, 0, Faction::Player);
    let mut enemy = ENPC::new("Enemy 1", 50, 10, 2, Faction::Enemy);
    
    let mut turn = 1;    
    while player.alive() && enemy.alive() {
	println!("\n\n*turn {}*", turn);
	player.readout();
	minion.readout();
	enemy.readout();
	println!("");
	
	player.attack(&mut enemy);
	// minion turn
	if minion.alive() && enemy.alive() {
	    minion.attack(&mut enemy);	
	}
	// enemy turn
	if enemy.alive() && minion.alive() {
	    enemy.attack(&mut minion);
	}
	else if enemy.alive() {
	    enemy.attack(&mut player);
	}
	turn += 1;
    }

    // End game logic
    if player.alive() {
	println!("You win!");
    }
    else {
	println!("You lose!");
    }
}
