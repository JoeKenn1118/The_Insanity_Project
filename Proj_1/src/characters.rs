use crate::general_info::*;
pub mod player {
    pub struct PlayerInfo {
        stats: super::stats::Stats,
        inventory: super::inventory::Inventory,
        equipped: super::inventory::Equipped,
    }

    pub fn get_player_stat (player: &PlayerInfo, stat: &str) -> i32 {
        super::stats::get_stat(&player.stats, stat)
    }
    
    pub fn create_player () -> PlayerInfo {
        PlayerInfo {
            stats: super::stats::Stats {
                strength: 10,
                dexterity: 10,
                constitution: 10,
                intelligence: 10,
                wisdom: 10,
                charisma: 10,
            },
            inventory: super::Inventory::Inventory {
                max_items: 10,
                items: Vec::new(),
            },
            equipped: super::Inventory::Equipped {
                weapon: super::Inventory::Item {
                    name: "Fists".to_string(),
                    enchantment: 0,
                    inventory: false,
                    bonus: 0,
                    value: 0,
                },
                off_hand: super::Inventory::Item {
                    name: "Fists".to_string(),
                    enchantment: 0,
                    inventory: false,
                    bonus: 0,
                    value: 0,
                },
                head: super::Inventory::Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    inventory: false,
                    bonus: 0,
                    value: 0,
                },
                chest: super::Inventory::Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    inventory: false,
                    bonus: 0,
                    value: 0,
                },
                legs: super::Inventory::Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    inventory: false,
                    bonus: 0,
                    value: 0,
                },
                feet: super::Inventory::Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    inventory: false,
                    bonus: 0,
                    value: 0,
                },
                hands: super::Inventory::Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    inventory: false,
                    bonus: 0,
                    value: 0,
                },
                ring1: super::Inventory::Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    inventory: false,
                    bonus: 0,
                    value: 0,
                },
                ring2: super::Inventory::Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    inventory: false,
                    bonus: 0,
                    value: 0,
                },
                amulet: super::Inventory::Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    inventory: false,
                    bonus: 0,
                    value: 0,
                },
                backpack: super::Inventory::Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    inventory: false,
                    bonus: 0,
                    value: 0,
                },
            }
        }
    }
}

pub mod monsters {
    pub struct MonsterInfo {
        stats: super::stats::Stats,
        inventory: super::inventory::Inventory,
        equipped: super::inventory::Equipped,
    }
}