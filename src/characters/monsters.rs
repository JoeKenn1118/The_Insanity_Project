/*
Scope for this has changed, before this was the go between for interacting with monsters. However now I'm thinking more that this should be a sort of encounter handler...

deprecating for now.
*/

use crate::general_info::{inventory::*, health::*, stats::*};

pub struct MonsterInfo {
    pub name: String,
    health: Health,
    stats: Stats,
    equipped: Equipped,
}

pub fn create_monster (monstertype: &str) -> MonsterInfo {
    match monstertype
    {
        "Orc" => orc::create_orc(),
        //"Goblin" => create_goblin(),
        _ => villager::create_villager()
    }
}

pub mod orc;
pub mod villager;