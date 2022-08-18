use crate::general_info::*;
pub mod player {
    pub struct PlayerInfo {
        pub name: String,
        stats: super::stats::Stats,
        inventory: super::inventory::Inventory,
        equipped: super::inventory::Equipped,
    }

    pub fn get_player_stat (player: &PlayerInfo, stat: &str) -> i32 {
        super::stats::get_stat(&player.stats, stat)
    }
    
    pub fn create_player () -> PlayerInfo {
        PlayerInfo {
            name: String::new(),
            stats: super::stats::init_stats(),
            inventory: super::inventory::init_inventory(),
            equipped: super::inventory::init_equipped(),
        }
    }

    pub fn set_player_name (player: &mut PlayerInfo, name: &str) {
        player.name = name.to_string();
    }

    pub fn player_skill_check (player: &PlayerInfo, skill: &str, difficulty: i32) -> bool {
        let skill_bonus: i32 = super::stats::get_stat_bonus(&player.stats, skill);
        let bonus :i32 = 0;
        super::actions::skill_check(skill_bonus, bonus, difficulty)
    }
}

pub mod monsters {
    pub struct MonsterInfo {
        stats: super::stats::Stats,
        inventory: super::inventory::Inventory,
        equipped: super::inventory::Equipped,
    }
}