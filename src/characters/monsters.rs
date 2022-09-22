/*
Scope for this has changed, before this was the go between for interacting with monsters. However now I'm thinking more that this should be a sort of encounter handler...

deprecating for now.
*/

use crate::general_info::{inventory::*, health::*, stats::*, actions::*};
use crate::characters::player::*;

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

    pub fn initiative_roll(&self) -> i32 {
        let dex_bonus: i32 = self.get_stat_bonus("dex");
        let bonus :i32 = 0; // Implement finding bonuses from equipped items
        super::actions::initiative_roll(dex_bonus, bonus)
    }

    pub fn get_stat_bonus(&self, stat: &str) -> i32 {
        let result = self.stats.get_stat_bonus(stat);
        return result;
    }

    pub fn get_health(&self) -> i32 {
        self.health.get_current_health()
    }

    pub fn get_armor_class(&self) -> i32 {
        self.equipped.get_armor_class()
    }

    pub fn armor_check(&self, roll: i32) -> bool {
        let armor_bonus: i32 = self.get_armor_class();
        if(roll >= armor_bonus) {
            return true;
        }
        else {
            return false;
        }
    }

    pub fn affect_health(&mut self, amount: i32) {
        self.health.affect_health(amount);
    }

    pub fn attack(&self, target: &mut PlayerInfo) {
        let str_bonus: i32 = self.get_stat_bonus("str");
        let bonus :i32 = 0; // Implement finding bonuses from equipped items
        let roll = super::actions::attack_roll(str_bonus, bonus);

        if(target.armor_check(roll)){
            let damage = self.equipped.get_item_equipped("Weapon").value;
            let weap_bonus = self.equipped.get_item_equipped("Weapon").bonus;
            let stat_bonus = self.get_stat_bonus("str");
            let tot_damage_bonus = weap_bonus + stat_bonus;
            let damage = damage_roll(tot_damage_bonus, damage);
            println!("{} hits {} for {} damage!", self.name, target.name, damage);
            target.affect_health(-damage);
        }
        else{
            println!("{} misses {}!", self.name, target.name);
        }
    }
}

pub mod orc;
pub mod villager;