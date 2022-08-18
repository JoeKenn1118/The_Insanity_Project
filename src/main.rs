mod characters;
mod general_info;

use std::io;
//use rand::Rng;    

use crate::general_info::*;
use crate::characters::player;

fn main() {
    let mut player: player::PlayerInfo = player::create_player();
    let mut name = String::new();
    let mut input = String::new();

    println!("Hello, and welcome to the game!");
    println!("Please enter your name: ");
    io::stdin().read_line(&mut name).expect("Failed to read line");

    player::set_player_name(&mut player, &name.trim());

    println!("Hello, {}!", player.name);
    println!("We begin in the Hamlet of Delvain. You are a mere peasant, with little to your name but drab clothes and calloused hands.");
    println!("Coming back to your village from chopping firewood, you see smoke drifting through the trees.");
    println!("Running now, you come up to Delvain, the once peaceful village, in flames.");
    println!("A figure Approaches, what do you do?");
    println!("[0] Creep around to get a better look");
    println!("[1] Approach Quickly to take the figure by surprise");
    println!("[2] Call out to the figure");
    println!("[3] Run away");
    println!("[4] Attack the figure");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input: u32 = input.trim().parse().expect("Please type a number!");

    match input {
        0 => {println!("You attempt to creep around the figure");},
        1 => {println!("You charge the figure your axe raised")},
        other => {println!("You do nothing")}
    }


}