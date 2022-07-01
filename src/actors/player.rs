use crate::actors::actor::Actor;
use crate::actors::actor::Faction;

// Player characters
pub struct PC {
    name: &'static str,
    hp:   u32,
    st:   u32,
    df:   u32,
    fact: Faction,
}

impl Actor for PC {
    fn new(name: &'static str, hp: u32, st: u32, df: u32, fact: Faction) -> Self {
	println!("Spawning {}\nhp: {}\nst: {}\n", name, hp, st);
	PC { name, hp, st, df, fact}
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

    fn defense(&self) -> u32 {
	self.df
    }

    fn faction(&self) -> Faction {
	self.fact
    }

    fn take_damage(&mut self, hp: u32) -> u32 {
	if hp >= self.health() {
	    self.hp = 0;
	    self.die();	    
	}
	else {
	    self.hp -= hp;	  
	};
	hp
    }
}
