use crate::general_info::*;
use super::*;
use rand::Rng;

pub fn create_orc () -> MonsterInfo{
    MonsterInfo {
        health: init_orc_health(),
        stats: init_orc_stats(),
        inventory: init_orc_inventory(),
        equipped: init_orc_equipped(),
    }
}

fn init_orc_health () -> health::Health {
    let val = rand::thread_rng().gen_range(5..=10);
    health::Health {
        total: val,
        current: val,
    }
}

fn init_orc_stats () -> stats::Stats {
    stats::Stats {
        strength: 13,
        dexterity: 8,
        constitution: 12,
        intelligence: 8,
        wisdom: 8,
        charisma: 6,
    }
}

fn init_orc_inventory () -> inventory::Inventory{
    inventory::Inventory {
        max_items: 10,
        items: Vec::<inventory::Item>::new(),
    }
}

fn init_orc_equipped () -> inventory::Equipped {
    let equip: inventory::Equipped = inventory::Equipped::init_equipped();

    // Set Specifics for Orc

    equip
        /*{
        weapon: inventory::Item {
            name: "Orc Sword".to_string(),
            enchantment: 0,
            bonus: 0,
            value: 0,
        },
        off_hand: inventory::Item {
            name: "None".to_string(),
            enchantment: 0,
            bonus: 0,
            value: 0,
        },
        armor: inventory::Armor{
            total_ac: 10,
            head: inventory::Item {
                name: "None".to_string(),
                enchantment: 0,
                bonus: 0,
                value: 0,
            },
            chest: inventory::Item {
                name: "None".to_string(),
                enchantment: 0,
                bonus: 0,
                value: 0,
            },
            legs: inventory::Item {
                name: "None".to_string(),
                enchantment: 0,
                bonus: 0,
                value: 0,
            },
            feet: inventory::Item {
                name: "None".to_string(),
                enchantment: 0,
                bonus: 0,
                value: 0,
            },
            hands: inventory::Item {
                name: "None".to_string(),
                enchantment: 0,
                bonus: 0,
                value: 0,
            },
        },
        ring1: inventory::Item {
            name: "None".to_string(),
            enchantment: 0,
            bonus: 0,
            value: 0,
        },
        ring2: inventory::Item {
            name: "None".to_string(),
            enchantment: 0,
            bonus: 0,
            value: 0,
        },
        amulet: inventory::Item {
            name: "Orc War Amulet".to_string(),
            enchantment: 0,
            bonus: 0,
            value: 0,
        },
        backpack.init_inventory(),
    }
    */
}