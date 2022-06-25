use std::collections::VecDeque;

pub trait Actor {
    // Instantiate an actor
    fn new(name: &'static str, hp: u32, st: u32) -> Self where Self: Sized;

    fn get_self(&self) -> Self where Self: Sized;

    // Return the actor's name
    fn name(&self) -> &'static str;

    // Return the actor's health
    fn health(&self) -> u32;

    // Return the actor's strength;
    fn strength(&self) -> u32;

    // Attack a target, Returns how much damage was dealt
    fn attack(&mut self, targ: &mut dyn Actor) -> u32;

    // Defend against an attack, Returns how much damage was taken
    fn defend(&mut self, targ: &mut dyn Actor) -> u32 {
	let (tst,sst) = (targ.strength(), self.strength());
	if tst > sst {
	    self.take_damage(tst - sst)
	}
	else if tst == sst {	    
	    self.take_damage(tst / 2);
	    targ.take_damage(sst / 2)
	}
	else {
	    targ.take_damage(sst - tst)
	}
    }

    // Take damage, Returns how much damage was taken
    fn take_damage(&mut self, hp: u32) -> u32;

    // Is the actor alive?
    fn alive(&self) -> bool {
	self.health() > 0
    }
    
    // Is the target actor friendly?
    fn friendly(&self, targ: &dyn Actor) -> bool;

    // Kill the actor
    // Returns nothing
    fn die(&self);

    // Takes a list of actors to interact with and takes its turn
    fn take_turn(&mut self, cs: &mut VecDeque<&dyn Actor>);
}



