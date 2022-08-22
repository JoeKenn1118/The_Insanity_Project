use crate::general_info::*;
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

fn init_villager_health () -> health::Health {
    let val = rand::thread_rng().gen_range(1..=5);
    health::Health {
        total: val,
        current: val,
    }
}

fn init_villager_stats () -> stats::Stats {
    stats::Stats {
        strength: 11,
        dexterity: 10,
        constitution: 9,
        intelligence: 8,
        wisdom: 10,
        charisma: 10,
    }
}

fn init_villager_inventory () -> inventory::Inventory{
    let mut inv: inventory::Inventory = inventory::init_inventory();

    inv.add_item(inventory::Item {
        name: "Gold".to_string(),
        enchantment: 0,
        inventory: false,
        bonus: 0,
        value: 1,
    });

    inv
}

fn init_villager_equipped () -> inventory::Equipped {
    let mut equip: inventory::Equipped = inventory::init_equipped();

    equip.weapon = inventory::Item {
        name: "Plow".to_string(),
        enchantment: 0,
        inventory: false,
        bonus: 0,
        value: 0,
    };
    equip.armor.chest = inventory::Item {
        name: "Peasant Shirt".to_string(),
        enchantment: 0,
        inventory: false,
        bonus: 0,
        value: 0,
    };
    equip.armor.legs = inventory::Item {
        name: "Peasant Trousers".to_string(),
        enchantment: 0,
        inventory: false,
        bonus: 0,
        value: 0,
    };
    equip.armor.feet = inventory::Item {
        name: "Peasant Shoes".to_string(),
        enchantment: 0,
        inventory: false,
        bonus: 0,
        value: 0,
    };

    inv
}