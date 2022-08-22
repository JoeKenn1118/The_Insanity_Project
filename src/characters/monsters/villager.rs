use crate::general_info::inventory::*;
use crate::general_info::health::*;
use crate::general_info::stats::*;
use super::*;
use rand::Rng;

pub fn create_villager () -> MonsterInfo{
    MonsterInfo {
        health: init_villager_health(),
        stats: init_villager_stats(),
        inventory: init_villager_inventory(),
        equipped: init_villager_equipped(),
    }
}

fn init_villager_health () -> Health {
    let val = rand::thread_rng().gen_range(1..=5);
    Health {
        total: val,
        current: val,
    }
}

fn init_villager_stats () -> Stats {
    Stats {
        strength: 11,
        dexterity: 10,
        constitution: 9,
        intelligence: 8,
        wisdom: 10,
        charisma: 10,
    }
}

pub fn init_villager_inventory () -> Inventory {
    let mut inv:Inventory = Inventory::init_inventory();
    // Add Changes to inventory
    let temp_item = Item{
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

    let mut temp_item: Item = Item {
        name: "Plow".to_string(),
        enchantment: 0,
        bonus: 0,
        value: 0,
    };
    equip.add_equipped("Weapon", temp_item);

    let mut temp_item: Item = Item {
        name: "Peasant Shirt".to_string(),
        enchantment: 0,
        bonus: 0,
        value: 0,
    };
    equip.add_equipped("Armor", temp_item);
    
    let mut temp_item: Item = Item {
        name: "Peasant Trousers".to_string(),
        enchantment: 0,
        bonus: 0,
        value: 0,
    };
    equip.add_equipped("Armor", temp_item);
    
    let mut temp_item: Item = Item {
        name: "Peasant Shoes".to_string(),
        enchantment: 0,
        bonus: 0,
        value: 0,
    };
    equip.add_equipped("Armor", temp_item);

    equip
}