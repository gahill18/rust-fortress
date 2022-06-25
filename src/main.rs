use std::collections::VecDeque;
use crate::actors::actor::Actor;

mod actors;
mod ui;
mod world;

fn main() {
    let hp: u32 = 100;
    let st: u32 = 10;
    let mut actors: VecDeque<&dyn Actor> = VecDeque::new();
    
    let pc = actors::player::PC::new("p1", hp, st);
    println!("hello player {}!", pc.name());
    actors.push_front(&pc);

    let fnpc = actors::friendly::FNPC::new("f1", hp, st);
    let enpc = actors::enemy::ENPC::new("e1", hp, st);
    println!("Ally: {}, Enemy: {}", fnpc.name(), enpc.name());
    actors.push_front(&fnpc);
    actors.push_front(&enpc);

    // As long as there are active actors, keep going
    while !actors.is_empty() {
	// Whoever is at the front of the queue gets to take their turn
	match actors.pop_front() {
	    Some(c) => c.get_self().take_turn(&mut actors),
	    None => (),
	}
    }
}
