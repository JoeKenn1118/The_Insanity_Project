use crate::general_info::*;

// Consider moving most of this to a character module which player builds on top of
pub mod player {
    use crate::general_info::{inventory::*, health::*, stats::*, actions::*};


    use super::monsters::MonsterInfo;
    pub struct PlayerInfo {
        health: super::health::Health,
        pub name: String,
        stats: super::stats::Stats,
        equipped: super::inventory::Equipped,
    }

    impl PlayerInfo {
        pub fn init_PlayerInfo(name: String) -> Self {
            Self {
                health: PlayerInfo::init_player_health(),
                name: name,
                stats: PlayerInfo::init_player_stats(),
                equipped: PlayerInfo::init_player_equipped()
            }
        }

        pub fn init_player_health () -> Health {
            let mut Hp = Health::init_health(50, 50);
            Hp.set_current_health(50);
            Hp.set_max_health(50);
    
            return Hp;
        }

        pub fn init_player_stats () -> Stats {
            let stats: Stats = Stats::init_stats(11,10,9,8,10,10);
    
            return stats;
        }

        pub fn init_player_equipped () -> Equipped {
            let mut equipped: Equipped = Equipped::init_equipped();

            let mut tempItem = Item::init_item();
            tempItem.name = "Rusty Axe".to_string();
            tempItem.bonus = 2;
            tempItem.value = 6;
            equipped.add_equipped("Weapon", tempItem);
            return equipped;
        }

        pub fn get_stat_bonus (&self, stat: &str) -> i32 {
            let result = self.stats.get_stat_bonus(stat);
            return result;
        }

        pub fn get_health(&self) -> i32 {
            self.health.get_current_health()
        }

        pub fn skill_check (&self, skill: &str, difficulty: i32) -> bool {
            let skill_bonus: i32 = self.get_stat_bonus(skill);
            let bonus :i32 = 0; // Implement finding bonuses from equipped items
            super::actions::skill_check(skill_bonus, bonus, difficulty)
        }

        pub fn initiative_roll(&self) -> i32 {
            let dex_bonus: i32 = self.get_stat_bonus("dex");
            let bonus :i32 = 0; // Implement finding bonuses from equipped items
            super::actions::initiative_roll(dex_bonus, bonus)
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

        pub fn attack(&self, monster: &mut MonsterInfo) { //if true, moster is dead
            let str_bonus: i32 = self.get_stat_bonus("str");
            let bonus :i32 = self.equipped.get_item_equipped("Weapon").bonus;
            let roll = super::actions::attack_roll(str_bonus, bonus);

            if(monster.armor_check(roll)){
                let damage = self.equipped.get_item_equipped("Weapon").value;
                let weap_bonus = self.equipped.get_item_equipped("Weapon").bonus;
                let stat_bonus = self.get_stat_bonus("str");
                let tot_damage_bonus = weap_bonus + stat_bonus;
                let damage = damage_roll(tot_damage_bonus, damage);
                println!("You hit {} for {} damage!", monster.name, damage);
                monster.affect_health(-damage);
            }
            else{
                println!("You missed!");
            }
        }
    }

    

    
}

pub mod monsters;