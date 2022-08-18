pub mod actions{
    pub use rand::Rng;
    pub fn skill_check (skill: i32, bonus: i32, difficulty: i32) -> bool {
        let roll = rand::thread_rng().gen_range(1..=20);
        if (skill + roll + bonus) > difficulty {
            return true;
        }
        return false;
    }
}

pub mod stats{
    pub struct Stats {
        strength: i32,
        dexterity: i32,
        constitution: i32,
        intelligence: i32,
        wisdom: i32,
        charisma: i32,
    }
    
    pub fn init_stats () -> Stats {
        Stats{
            strength: 10,
            dexterity: 10,
            constitution: 10,
            intelligence: 10,
            wisdom: 10,
            charisma: 10,
        }
    }

    pub fn get_stat (stats: &Stats, stat_name: &str) -> i32 {
        match stat_name {
            "strength" => stats.strength,
            "dexterity" => stats.dexterity,
            "constitution" => stats.constitution,
            "intelligence" => stats.intelligence,
            "wisdom" => stats.wisdom,
            "charisma" => stats.charisma,
            _ => 1,
        }
    }

    pub fn get_stat_bonus (stats: &Stats, stat_name: &str) -> i32 {
        let stat = get_stat(stats, stat_name);
        return (stat - 10) / 2;
    }

    pub fn set_stat (stats: &mut Stats, stat_name: &str, value: i32) {
        match stat_name {
            "strength" => stats.strength = value,
            "dexterity" => stats.dexterity = value,
            "constitution" => stats.constitution = value,
            "intelligence" => stats.intelligence = value,
            "wisdom" => stats.wisdom = value,
            "charisma" => stats.charisma = value,
            _ => (),
        }
    }
}

pub mod inventory{
    pub struct Item {
        name: String,
        enchantment: i32,
        inventory: bool,
        bonus: i32,
        value: i32,
    }
    pub struct Equipped {
        weapon: Item,
        off_hand: Item,
        head: Item,
        chest: Item,
        legs: Item,
        feet: Item,
        hands: Item,
        ring1: Item,
        ring2: Item,
        amulet: Item,
        backpack: Item,
    }
    pub struct Inventory {
        max_items: i32,
        items: Vec<Item>,
    }

    pub fn init_inventory () -> Inventory {
        Inventory {
            max_items: 10,
            items: Vec::new(),
        }
    }

    pub fn init_equipped () -> Equipped {
        Equipped {
            weapon: Item {
                name: "Axe".to_string(),
                enchantment: 0,
                inventory: false,
                bonus: 1,
                value: 0,
            },
            off_hand: Item {
                name: "None".to_string(),
                enchantment: 0,
                inventory: false,
                bonus: 0,
                value: 0,
            },
            head: Item {
                name: "None".to_string(),
                enchantment: 0,
                inventory: false,
                bonus: 0,
                value: 0,
            },
            chest: Item {
                name: "None".to_string(),
                enchantment: 0,
                inventory: false,
                bonus: 0,
                value: 0,
            },
            legs: Item {
                name: "None".to_string(),
                enchantment: 0,
                inventory: false,
                bonus: 0,
                value: 0,
            },
            feet: Item {
                name: "None".to_string(),
                enchantment: 0,
                inventory: false,
                bonus: 0,
                value: 0,
            },
            hands: Item {
                name: "None".to_string(),
                enchantment: 0,
                inventory: false,
                bonus: 0,
                value: 0,
            },
            ring1: Item {
                name: "None".to_string(),
                enchantment: 0,
                inventory: false,
                bonus: 0,
                value: 0,
            },
            ring2: Item {
                name: "None".to_string(),
                enchantment: 0,
                inventory: false,
                bonus: 0,
                value: 0,
            },
            amulet: Item {
                name: "None".to_string(),
                enchantment: 0,
                inventory: false,
                bonus: 0,
                value: 0,
            },
            backpack: Item {
                name: "None".to_string(),
                enchantment: 0,
                inventory: false,
                bonus: 0,
                value: 0,
            },
        }
    }
}