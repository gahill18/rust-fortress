use std::collections::VecDeque;
use crate::actors::actor::Actor;

// Player characters
#[derive(Debug)]    
pub struct FNPC {
    name: &'static str,
    hp:   u32,
    st:   u32,
}

impl Actor for FNPC {
    fn new(name: &'static str, hp: u32, st: u32) -> Self {
	FNPC { name, hp, st, }
    }

    fn name(&self) -> &'static str {
	self.name
    }

    fn health(&self) -> u32 {
	self.hp
    }

    fn strength(&self) -> u32 {
	self.st
    }

    fn attack(&mut self, targ: &mut dyn Actor) -> u32 {
	if targ.alive() {
	    let dam = targ.defend(self);
	    println!("{} dealt {} damage to {}", self.name(), dam, targ.name());
	    dam
	}
	else { 0 }
    }

    fn take_damage(&mut self, hp: u32) -> u32 {
	if hp >= self.health() {
	    self.die();
	    self.health()
	}
	else {
	    self.hp -= hp;
	    hp
	}
    }

    fn friendly(&self, targ: &dyn Actor) -> bool {
	true
    }

    fn die(&self) {
	println!("{} is dead!", self.name());
    }

    fn take_turn(&mut self, actors: &mut VecDeque<&dyn Actor>) {
	
    }
}
