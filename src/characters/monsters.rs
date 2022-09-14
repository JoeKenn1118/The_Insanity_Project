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

impl MonsterInfo {
    pub fn create_monster (monstertype: &str) -> Self {
        match monstertype
        {
            "Orc" => orc::create_orc(),
            //"Goblin" => create_goblin(),
            _ => villager::create_villager()
        }
    }

    pub fn monster_initiative_roll(&self) -> i32 {
        let dex_bonus: i32 = self.get_monster_stat_bonus("dex");
        let bonus :i32 = 0; // Implement finding bonuses from equipped items
        super::actions::initiative_roll(dex_bonus, bonus)
    }

    pub fn get_monster_stat_bonus(&self, stat: &str) -> i32 {
        let result = self.stats.get_stat_bonus(stat);
        return result;
    }
}

pub mod orc;
pub mod villager;