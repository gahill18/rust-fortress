use crate::actors::actor::*;
use crate::actors::actor::Faction;

// Friendly characters
pub struct FNPC {
    name: &'static str,
    hp:   u32,
    st:   u32,
    df:   u32,
    fact: Faction
}

impl Actee for FNPC {
    fn new(name: &'static str, hp: u32, st: u32, df: u32, fact: Faction) -> Self {
	println!("Spawning {}\nhp: {}\nst: {}\ndf: {}\nfaction:{:?}\n", name, hp, st, df, fact);
	FNPC { name, hp, st, df, fact}
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

impl Actor for FNPC {
}

impl Moveable for FNPC {
}
