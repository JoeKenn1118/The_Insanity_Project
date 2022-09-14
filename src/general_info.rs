pub fn end_game() {
    println!("Game Over, Thanks for playing!");
    std::process::exit(0);
}

use std::io;
pub fn selection() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().parse().expect("Please type a number!")
}

pub mod actions{
    //use std::vec;

    use rand::Rng;
    use crate::characters::{monsters::*, player::*, self};

    pub fn skill_check (skill: i32, bonus: i32, difficulty: i32) -> bool {
        let roll = rand::thread_rng().gen_range(1..=20);
        if (skill + roll + bonus) > difficulty {
            return true;
        }
        return false;
    }

    pub struct Position{
        pub name: String,
        pub initiative: i32
    }

    pub fn initiative_roll(dex_bonus:i32, bonus: i32) -> i32 {
        let roll = rand::thread_rng().gen_range(1..=20);
        return roll + dex_bonus + bonus;
    }

    pub fn attack_roll (str_bonus: i32, bonus: i32) -> i32 {
        let roll = rand::thread_rng().gen_range(1..=20);
        return roll + str_bonus + bonus;
    }

    // Combat! if false, player is dead
    pub fn combat (player: &mut PlayerInfo, monster: &mut MonsterInfo, player_surprise: bool, monster_surprise: bool) -> bool {
        let mut order: Vec<Position> = Vec::new();
        order.push(Position{name: player.name.clone(), initiative: player.player_initiative_roll()});
        order.push(Position{name: monster.name.clone(), initiative: monster.monster_initiative_roll()});
        
        order.sort_by(|a, b| b.initiative.cmp(&a.initiative));

        if(player_surprise){
            if(monster_surprise){
                panic!("Both players cannot be surprised!");
            }
            println!("You get the first attack!");
            let mut result = player.player_attack(monster);
        }

        for turn in order.len()..0 {
            println!("It is {}'s turn!", order[turn].name);
            
            if order[turn].name == player.name {
            }
        }

        println!("Combat!");
        println!("You approach the {} and with a swift heave of his sword you are cleaved nearly in two.", monster.name);

        return false;
    }
}

pub mod health{
    pub struct Health {
        pub max: i32,
        current: i32,
    }
    impl Health {
        pub fn init_health (max: i32, current: i32) -> Self {
            if max < 1 || current < 1 {
                panic!("Max or Current health cannot be less than 1 on init");
            }
            else if current > max {
                panic!("Current health cannot be larger than max on init");
            }          
            Self {
                max: max,
                current: current,
            }
        }
        pub fn set_max_health (&mut self, max: i32) {
            self.max = max;
        }
        pub fn set_current_health (&mut self, current: i32) {
            self.current = current;
        }
        pub fn get_current_health (&self) -> i32 {
            self.current
        }
        pub fn affect_health (&mut self, amount: i32) -> i32 {
            self.current += amount;
            self.current
        }
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
    
    impl Stats {
        pub fn init_stats (str: i32, dex: i32, con: i32, int: i32, wis: i32, cha: i32) -> Self {
            Self {
                strength: str,
                dexterity: dex,
                constitution: con,
                intelligence: int,
                wisdom: wis,
                charisma: cha,
            }
        }

        pub fn get_stat (&self, stat_name: &str) -> i32 {
            match stat_name {
                "str" => self.strength,
                "dex" => self.dexterity,
                "con" => self.constitution,
                "int" => self.intelligence,
                "wis" => self.wisdom,
                "cha" => self.charisma,
                _ => panic!("Invalid stat name"),
            }
        }

        pub fn get_stat_bonus (&self, stat: &str) -> i32 {
            let mut result = self.get_stat(stat);
            result = (result - 10) / 2;

            return result;
        }

        pub fn set_stat (stats: &mut Stats, stat_name: &str, value: i32) {
            match stat_name {
                "str" => stats.strength = value,
                "dex" => stats.dexterity = value,
                "con" => stats.constitution = value,
                "int" => stats.intelligence = value,
                "wis" => stats.wisdom = value,
                "cha" => stats.charisma = value,
                _ => (),
            }
        }
    }
}

pub mod inventory{
    use std::io;

    #[derive(Clone)]
    pub struct Item {
        pub name: String,
        pub enchantment: i32,
        pub bonus: i32,
        pub value: i32,
    }

    impl Item {
        pub fn init_item () -> Self {
            Self {
                name: "None".to_string(),
                enchantment: 0,
                bonus: 0,
                value: 0,
            }
        }
    }
    pub struct Armor {
        total_ac: i32,
        head: Item,
        chest: Item,
        legs: Item,
        feet: Item,
        hands: Item,
    }

    impl Armor {
        pub fn init_armor() ->Self {
            Self {
                total_ac: 10,
                head: Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    bonus: 0,
                    value: 0
                },
                chest: Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    bonus: 0,
                    value: 0
                },
                legs: Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    bonus: 0,
                    value: 0
                },
                feet: Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    bonus: 0,
                    value: 0
                },
                hands: Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    bonus: 0,
                    value: 0
                }
            }
        }
    }
    pub struct Equipped {
        weapon: Item,
        off_hand: Item,
        armor: Armor,
        ring1: Item,
        ring2: Item,
        amulet: Item,
        pub bag: Inventory,
    }

    impl Equipped{
        pub fn init_equipped () -> Self{
                Self{
                    weapon: Item {
                        name: "None".to_string(),
                        enchantment: 0,
                        bonus: 0,
                        value: 0,
                    },

                    off_hand : Item {
                        name: "None".to_string(),
                        enchantment: 0,
                        bonus: 0,
                        value: 0,
                    },

                    armor: Armor::init_armor(),

                    ring1: Item {
                        name: "None".to_string(),
                        enchantment: 0,
                        bonus: 0,
                        value: 0,
                    },

                    ring2: Item {
                        name: "None".to_string(),
                        enchantment: 0,
                        bonus: 0,
                        value: 0,
                    },

                    amulet: Item {
                        name: "None".to_string(),
                        enchantment: 0,
                        bonus: 0,
                        value: 0,
                    },

                    bag: Inventory::init_inventory()
                }
            }
           
        pub fn add_equipped (&mut self, item_type: &str, item: Item) {
            if self.find_equipped(item_type) {
                self.remove_equipped(item_type);
            }
            match item_type {
                "Weapon" => self.weapon = item,
                "Off Hand" => self.off_hand = item,
                "Armor" => (),
                "Ring 1" => self.ring1 = item,
                "Ring 2" => self.ring2 = item,
                "Amulet" => self.amulet = item,
                _ => ()
            }
        }

        fn find_equipped (&mut self, item_type: &str) -> bool {
            match item_type {
                "Weapon" => {
                    if self.weapon.name == "None" {
                        return false
                    }
                    return true
                },
                "Off Hand" => {
                    if self.off_hand.name == "None" {
                        return false
                    }
                    return true
                },
                "Armor" => return false, // Figure this out
                "Ring 1" => {
                    if self.ring1.name == "None" {
                        return false
                    }
                    return true
                }
                "Ring 2" => {
                    if self.ring2.name == "None" {
                        return false
                    }
                    return true
                }
                "Amulet" => {
                    if self.amulet.name == "None" {
                        return false
                    }
                    return true
                }
                _ => {println!("Incorrect Item Type");
                        return false
                    },
            }
        }
        
        fn remove_equipped(&mut self, item_type: &str) {
            match item_type {
                "Weapon" => {
                    if self.bag.add_item(self.weapon.clone()) 
                    {
                        self.weapon = Item {
                            name: "None".to_string(),
                            enchantment: 0,
                            bonus: 0,
                            value: 0,
                            };
                    }
                    else {
                        println!("Would you like to drop your {}?", self.weapon.name)
                    }
                    
                },
                "Off Hand" => self.off_hand = Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    bonus: 0,
                    value: 0
                },
                "Armor" => (),
                "Ring 1" => self.ring1 = Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    bonus: 0,
                    value: 0,
                },
                "Ring 2" => self.ring2 = Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    bonus: 0,
                    value: 0,
                },
                "Amulet" => self.amulet = Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    bonus: 0,
                    value: 0,
                },
                _ => (),
            }
        }

        pub fn print_equipped(&self) {
            println!("Weapon: {}", self.weapon.name);
            println!("Off Hand: {}", self.off_hand.name);
            println!("Armor: {}", self.armor.total_ac);
            println!("Ring 1: {}", self.ring1.name);
            println!("Ring 2: {}", self.ring2.name);
            println!("Amulet: {}", self.amulet.name);
        }

        pub fn get_item_equipped(&self, item_type: &str) -> Item {
            match item_type {
                "Weapon" => self.weapon.clone(),
                "Off Hand" => self.off_hand.clone(),
                "Armor" => Item::init_item(),
                "Ring 1" => self.ring1.clone(),
                "Ring 2" => self.ring2.clone(),
                "Amulet" => self.amulet.clone(),
                _ => Item::init_item(),
            }
        }
    }
    pub struct Inventory {
        max_items: usize,
        items: Vec<Item>,
    }

    impl Inventory {
        pub fn init_inventory () -> Self {
            Self {
                max_items: 0,
                items: Vec::<Item>::new()
            }
        }

        pub fn increase_max_items (&mut self, max_items: usize) {
            self.max_items = max_items;
        }

        pub fn add_item (&mut self, item: Item) -> bool {
            if self.max_items < self.items.len() + 1{
                self.items.push(item);
                return true
            }
            else {
                println!("Inventory Full");
                println!("Would you like to drop {}? 1 - Yes, 0 - No", item.name);

                let mut input = String::new();

                io::stdin().read_line(&mut input).expect("Failed to read line");

                let input: u32 = input.trim().parse().expect("Please type a number!");

                if input == 1 {
                    println!("{}, dropped.", item.name);
                    return true
                }
                println!("{}, cannot be added to inventory!", item.name);
                return false
            }
            
        }
        pub fn remove_item (&mut self, item: &Item) {
            
            if self.find_item(item.name.as_str()) {
                self.items.remove(self.find_item_index(item.name.as_str()));
            }
            else {
                println!("Item not found");
            }
        }
        pub fn get_item (&self, item: &str) -> Item {
            if self.find_item(item) {
                let index = self.find_item_index(item);
                return self.items[index].clone()
            }
            else {
                println!("Item not found");
                return Item {
                    name: "".to_string(),
                    enchantment: 0,
                    bonus: 0,
                    value: 0,
                }
            }
        }
        fn find_item (&self, item: &str) -> bool {
            for i in self.items.iter() {
                if item == i.name {
                    return true;
                }
            }
            return false;
        }
        fn find_item_index(&self, item: &str) -> usize {
            let mut iterator: usize = 0;
            for i in self.items.iter() {
                if item == i.name {
                    break
                }
                iterator += 1;
            }
            iterator
        }
    }
}