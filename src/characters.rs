use crate::general_info::*;

// Consider moving most of this to a character module which player builds on top of
pub mod player {
    use crate::general_info::inventory::Equipped;
    use crate::general_info::health::*;
    use crate::general_info::stats::*;
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
                equipped: Equipped::init_equipped()
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

        pub fn get_player_stat_bonus (&self, stat: &str) -> i32 {
            let result = self.stats.get_stat_bonus(stat);
            return result;
        }

        pub fn player_skill_check (&self, skill: &str, difficulty: i32) -> bool {
            let skill_bonus: i32 = self.get_player_stat_bonus(skill);
            let bonus :i32 = 0; // Implement finding bonuses from equipped items
            super::actions::skill_check(skill_bonus, bonus, difficulty)
        }
    }

    

    
}

pub mod monsters;