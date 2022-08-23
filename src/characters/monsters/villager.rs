use crate::general_info::inventory::*;
use crate::general_info::health::*;
use crate::general_info::stats::*;
use super::*;
use rand::Rng;

pub fn create_villager () -> MonsterInfo{
    MonsterInfo {
        name: "Villager".to_string(),
        health: init_villager_health(),
        stats: init_villager_stats(),
        equipped: init_villager_equipped(),
    }
}

fn init_villager_health () -> Health {
    let val = rand::thread_rng().gen_range(1..=5);
    let Hp = Health::init_health(val, val);

    Hp
}

fn init_villager_stats () -> Stats {
    let stats: Stats = Stats::init_stats(11,10,9,8,10,10);

    stats
}

pub fn init_villager_inventory () -> Inventory {
    let mut inv:Inventory = Inventory::init_inventory();
    // Add Changes to inventory
    let temp_item = Item {
            name: "Gold".to_string(),
            enchantment: 0,
            bonus: 0,
            value: 1,
    };
    inv.add_item(temp_item);
    return inv;
}

fn init_villager_equipped () -> Equipped {
    let mut equip: Equipped = Equipped::init_equipped();

    let temp_item: Item = Item {
        name: "Plow".to_string(),
        enchantment: 0,
        bonus: 0,
        value: 0,
    };
    equip.add_equipped("Weapon", temp_item);

    let temp_item: Item = Item {
        name: "Peasant Shirt".to_string(),
        enchantment: 0,
        bonus: 0,
        value: 0,
    };
    equip.add_equipped("Armor", temp_item);
    
    let temp_item: Item = Item {
        name: "Peasant Trousers".to_string(),
        enchantment: 0,
        bonus: 0,
        value: 0,
    };
    equip.add_equipped("Armor", temp_item);
    
    let temp_item: Item = Item {
        name: "Peasant Shoes".to_string(),
        enchantment: 0,
        bonus: 0,
        value: 0,
    };
    equip.add_equipped("Armor", temp_item);

    equip.bag.increase_max_items(1);

    let temp_item: Item = Item {
        name: "Gold".to_string(),
        enchantment: 0,
        bonus: 0,
        value: 1,
    };

    equip.bag.add_item(temp_item);

    equip
}