use super::*;
use rand::Rng;
use crate::general_info::{inventory::*, health::*, stats::*};

pub fn create_orc () -> MonsterInfo{
    MonsterInfo {
        name: "Orc".to_string(),
        health: init_orc_health(),
        stats: init_orc_stats(),
        equipped: init_orc_equipped(),
    }
}

fn init_orc_health () -> Health {
    let val = rand::thread_rng().gen_range(5..=10);
    let Hp = Health::init_health(val, val);

    Hp
}

fn init_orc_stats () -> Stats {
    let stats: Stats = Stats::init_stats(11,10,9,8,10,10);

    stats
}

fn init_orc_inventory () -> Inventory{
     Inventory::init_inventory()
    // Add Changes to inventory
}

fn init_orc_equipped () -> Equipped {
    let mut equip: Equipped = Equipped::init_equipped();

    // Set Specifics for Orc
    let temp_item: Item = Item {
        name: "Orc Sword".to_string(),
            enchantment: 0,
            bonus: 1,
            value: 6,
    };
    equip.add_equipped("Weapon", temp_item);

    let temp_item: Item = Item {
        name: "Orc War Amulet".to_string(),
            enchantment: 0,
            bonus: 0,
            value: 0,
    };
    equip.add_equipped("Amulet", temp_item);


    let temp_item: Item = Item {
        name: "Gold".to_string(),
            enchantment: 0,
            bonus: 0,
            value: 10,
    };
    equip.bag.add_item(temp_item);
    equip
       
}