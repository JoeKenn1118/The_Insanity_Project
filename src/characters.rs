use crate::general_info::*;
pub mod player {
    pub struct PlayerInfo {
        health: super::health::Health,
        pub name: String,
        stats: super::stats::Stats,
        inventory: super::inventory::Inventory,
        equipped: super::inventory::Equipped,
    }

    pub fn get_player_stat_bonus (player: &PlayerInfo, stat: &str) -> i32 {
        super::stats::get_stat_bonus(&player.stats, stat)
    }
    
    pub fn create_player () -> PlayerInfo {
        PlayerInfo {
            health: init_player_health(),
            name: String::new(),
            stats: super::stats::init_stats(),
            inventory: super::inventory::init_inventory(),
            equipped: super::inventory::init_equipped(),
        }
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