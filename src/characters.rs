use crate::general_info::*;

// Consider moving most of this to a character module which player builds on top of
pub mod player {
    use crate::general_info::inventory::Equipped;

    pub struct PlayerInfo {
        health: super::health::Health,
        pub name: String,
        stats: super::stats::Stats,
        equipped: super::inventory::Equipped,
    }

    impl PlayerInfo {
        pub fn init_PlayerInfo() -> Self {
            Self {
                health: init_player_health(),
                name: String::new(),
                stats: super::stats::init_stats(),
                equipped: Equipped::init_equipped()
            }
        }
    }

    pub fn get_player_stat_bonus (player: &PlayerInfo, stat: &str) -> i32 {
        super::stats::get_stat_bonus(&player.stats, stat)
    }
    
    pub fn create_player () -> PlayerInfo {
        let mut player: PlayerInfo = PlayerInfo::init_PlayerInfo();
        
        player
    }

    pub fn init_player_health () -> super::health::Health {
        super::health::Health {
            total: 20,
            current: 20,
        }
    }

    pub fn set_player_name (player: &mut PlayerInfo, name: &str) {
        player.name = name.to_string();
    }

    pub fn player_skill_check (player: &PlayerInfo, skill: &str, difficulty: i32) -> bool {
        let skill_bonus: i32 = get_player_stat_bonus(&player, skill);
        let bonus :i32 = 0;
        super::actions::skill_check(skill_bonus, bonus, difficulty)
    }
}

mod monsters;