use crate::actors::actor::Actor;
use crate::actors::actor::Faction;

// Enemy characters
pub struct ENPC {
    name: &'static str,             // unique name
    hp:   u32,                      // health
    st:   u32,                      // strength
    df:   u32,                      // defense
    fact: Faction,
}

impl Actor for ENPC {
    fn new(name: &'static str, hp: u32, st: u32, df: u32, fact: Faction) -> Self {
	println!("Spawning {}\nhp: {}\nst: {}\ndf: {}", name, hp, st, df);
	ENPC { name, hp, st, df, fact }
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

    fn friendly(&self, targ: &dyn Actor) -> bool {
	match targ.faction() {
	    Player => false,
	    Enemy  => true,
	}
    } 
}
