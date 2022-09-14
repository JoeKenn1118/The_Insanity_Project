use crate::general_info::*;

// Consider moving most of this to a character module which player builds on top of
pub mod player {
    use crate::general_info::inventory::Equipped;
    use crate::general_info::inventory::*;
    use crate::general_info::health::*;
    use crate::general_info::stats::*;

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
            equipped.add_equipped("Weapon", tempItem);
            return equipped;
        }

        pub fn get_player_stat_bonus (&self, stat: &str) -> i32 {
            let result = self.stats.get_stat_bonus(stat);
            return result;
        }

        pub fn player_skill_check (&self, skill: &str, difficulty: i32) -> bool {
            let skill_bonus: i32 = self.get_player_stat_bonus(skill);
            let bonus :i32 = 0; // Implement finding bonuses from equipped items
            super::actions::skill_check(skill_bonus, bonus, difficulty)
        }

        pub fn player_initiative_roll(&self) -> i32 {
            let dex_bonus: i32 = self.get_player_stat_bonus("dex");
            let bonus :i32 = 0; // Implement finding bonuses from equipped items
            super::actions::initiative_roll(dex_bonus, bonus)
        }

        pub fn player_attack(&self, monster: &mut MonsterInfo) {
            let str_bonus: i32 = self.get_player_stat_bonus("str");
            let bonus :i32 = self.equipped.get_item_equipped("Weapon").bonus;
            let roll = super::actions::attack_roll(str_bonus, bonus);
        }
    }

    

    
}

pub mod monsters;