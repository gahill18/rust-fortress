// Things that can be acted upon
pub trait Actee {
    // Instantiate an actor
    fn new(name: &'static str, hp: u32, st: u32, df: u32, fact: Faction) -> Self where Self: Sized;
    // Return the actor's name
    fn name(&self)      -> &'static str;
    // Return the actor's health
    fn health(&self)    -> u32     { 1 }
    // Return the actor's strength
    fn strength(&self)  -> u32     { 0 }
    // Return the actor's defense
    fn defense(&self)   -> u32     { 0 }
    fn faction(&self)   -> Faction { Faction::Neutral }
    // Take damage in a way that will impact the return value of self.health()
    // Returns how much damage was taken
    fn take_damage(&mut self, hp: u32) -> u32;
    // Is the actor alive?
    fn alive(&self) -> bool {
	self.health() > 0
    }
    // Is the target actor friendly?
    fn friendly(&self, targ: &dyn Actor) -> bool {
	self.faction() == targ.faction()
    }
    // Kill the actor. Returns nothing
    fn die(&self) {
	println!("{} has died!", self.name());
    }
    // Print current stats
    fn readout(&self) {
	println!("{}, hp: {}, st: {}, df: {}, faction: {:?}",
		 self.name(), self.health(), self.strength(), self.defense(), self.faction());
    }
}

// Anything that needs to make decisions each turn, i.e. "Acts"
// Actors must implement Actee (anything that can act on others must be able to be acted on)
// Includes default implementations where possible/reasonable
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

// Necessary functions for controlling a player character
pub trait Playable {
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Faction {
    Player,
    Enemy,
    Neutral
}
