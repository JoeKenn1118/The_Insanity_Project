pub mod stats{
    pub struct Stats {
        strength: i32,
        dexterity: i32,
        constitution: i32,
        intelligence: i32,
        wisdom: i32,
        charisma: i32,
    }
    
    pub fn get_stat (stat_set: &Stats, stat: &str) -> i32 {
        match stat {
            "strength" => stat_set.strength,
            "dexterity" => stat_set.dexterity,
            "constitution" => stat_set.constitution,
            "intelligence" => stat_set.intelligence,
            "wisdom" => stat_set.wisdom,
            "charisma" => stat_set.charisma,
            _ => 0,
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
}