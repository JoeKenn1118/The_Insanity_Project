mod characters;
mod general_info;

use std::io;
//use rand::Rng;    

//use crate::general_info::*;
use crate::{characters::*, general_info::actions::{self, combat}};

fn main() {
    let mut name = String::new();
    let mut input: u32;
    let mut difficulty: i32;

    println!("Hello, and welcome to the game!");
    println!("Please enter your name: ");
    io::stdin().read_line(&mut name).expect("Failed to read line");

    let mut player: player::PlayerInfo = player::PlayerInfo::init_PlayerInfo(name.trim().to_string());

    println!("Hello, {}!", player.name);
    println!("We begin in the Hamlet of Delvain. You are a mere peasant, with little to your name but drab clothes and calloused hands.");
    println!("Coming back to your village from chopping firewood, you see smoke drifting through the trees.");
    println!("Running now, you come up to Delvain, the once peaceful village, in flames.");
    println!("A figure Approaches, what do you do?");
    println!("[0] Creep around to get a better look");
    println!("[1] Approach Quickly to take the figure by surprise");
    println!("[2] Call out to the figure");
    println!("[3] Run away");

    input = selection();

    let mut result = false;
    let mut Orc = monsters::create_monster("Orc");
    match input {
        0 => {
            println!("You attempt to creep around the figure");
            difficulty = 12;
            result = player.player_skill_check("dex", difficulty);
            if result {
                println!("You successfully creep around the figure");
                println!("The figure is a {}!", Orc.name);
                println!("What do you do?");
                println!("[0] Attack");
                println!("[1] Run away");
                println!("[2] Sneak past");
                println!("[3] Sneak up behind");
                input = selection();
                match input {
                    0 => {
                        println!("You charge at the {}!", Orc.name);
                        result = combat(&mut player, &mut Orc);
                    }
                    1 => {
                        println!("You run away!");
                        result = true;
                    }
                    2 => {
                        println!("You attempt to sneak past the {}!", Orc.name);
                        result = player.player_skill_check("dex", difficulty);

                        if result {
                            println!("You successfully sneak past the {}!", Orc.name);
                        } 
                        else {
                            println!("You failed to sneak past the {}!", Orc.name);
                            println!("\"You almost got past me human!\" You hear from behind.");
                            result = combat(&mut player, &mut Orc);
                        }
                    }
                    3 => {
                        println!("You attempt to sneak up behind the {}!", Orc.name);
                        result = player.player_skill_check("dex", difficulty);

                        if result {
                            println!("You successfully sneak up behind the {}!", Orc.name);
                            result = combat(&mut player, &mut Orc);
                        }
                        else {
                            println!("You failed to sneak up behind the {}!", Orc.name);
                            println!("\"You almost caught me human!\nAn effort at least, but it was for nothing!\"");
                            result = combat(&mut player, &mut Orc);
                        }
                    }
                    _ => {
                        println!("Invalid input");
                        result = false;
                    }
                }


            } else {
                println!("You failed to creep around the figure");
                println!("The figure turns to you. \"Pathetic, couldn't even muster the will to face me\"");
                result = combat(&mut player, &mut Orc);
            }
        },
        1 => {
            println!("You charge the figure, your axe raised");
            println!("The figure is a {}!", Orc.name);
            println!("\"Aha Human! What Fervour! Come at me!\"");
            result = combat(&mut player, &mut Orc);
        },
        2 => {
            println!("You call out to the figure, and it responds");
            println!("\"Ah, showing yourself I see.\" The figure turns, showing itself as an Orc!");
            println!("\"Come to your death peasant, at least it might be an honourable one.\"");
            result = combat(&mut player, &mut Orc);
        },
        3 => {
            println!("You run back into the woods")
        },
        _ => {
            println!("You do nothing")
        }
    }

    if result == false {
        println!("You lie face down in the dirt, blood pooling around your body as you fade from consciousness.");
        println!("You have died!");
        return ();
    }
    else {
        println!("You live to see another day");
        println!("Congratulations, you have won!");
    }

    println!("Thanks for playing!");

}

fn selection() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().parse().expect("Please type a number!")
}
