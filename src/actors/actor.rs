// This thing can be acted upon
pub trait Actee {
    // Instantiate an actor
    fn new(name: &'static str, hp: u32, st: u32, df: u32, fact: Faction) -> Self where Self: Sized;
    // Return the actor's name
    fn name(&self)      -> &'static str { "Default Actee Name" }
    // Return the actor's health
    fn health(&self)    -> u32          { 1 }
    // Return the actor's strength
    fn strength(&self)  -> u32          { 0 }
    // Return the actor's defense
    fn defense(&self)   -> u32          { 0 }
    fn faction(&self)   -> Faction      { Faction::Neutral }
    // Return how much damage in a way that will impact the return value of self.health()
    fn take_damage(&mut self, hp: u32) -> u32;
    // Is the actor alive?
    fn alive(&self) -> bool {
	self.health() > 0
    }
    // Is the target actor friendly?
    fn friendly(&self, targ: &dyn Actee) -> bool {
	self.faction() == targ.faction()
    }
    // Is the target actor hostile?
    fn hostile(&self, targ: &dyn Actee) -> bool {
	// they're only hostile if the actees arent friendly and neither is neutral
	!self.friendly(targ) && match (self.faction(), targ.faction()) {	    
	    (sf, tf) if sf == Faction::Neutral || tf == Faction::Neutral => true,
	    (_, _) => false,
	}
    }
    // Kill the actor
    fn die(&self) {
	println!("{} has died!", self.name());
    }
    // Print current stats
    fn readout(&self) {
	println!("{}, hp: {}, st: {}, df: {}, faction: {:?}",
		 self.name(), self.health(), self.strength(), self.defense(), self.faction());
    }
}

// This thing can perform actions on Actees, and is an Actee itself
pub trait Actor: Actee {
    // Attack a target, Returns how much damage was dealt
    fn attack(&mut self, targ: &mut dyn Actee) -> u32 {	
	if self.alive() && targ.alive() {
	    let (sst,tdf) = (self.strength(), targ.defense());
	    let dam = if sst > tdf { sst - tdf } else { 0 };
	    targ.take_damage(dam);
	    println!("{} attacked {} for {}", self.name(), targ.name(), dam);
	    dam
	}
	else { 0 }
    }
}

// This actor can move around the map
// This is entirely optional for non-player actors and actees
pub trait Moveable: Actor {
    fn move_up(&self) {
	println!("{} moving up", self.name());
    }

    fn move_down(&self) {
	println!("{} moving down", self.name());
    }

    fn move_left(&self) {
	println!("{} moving left", self.name());
    }

    fn move_right(&self) {
	println!("{} moving right", self.name());
    }
}

// The player can directly play as this thing 
pub trait Playable: Moveable {
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Faction {
    Player,
    Enemy,
    Neutral
}
