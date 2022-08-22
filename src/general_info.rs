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

pub mod health{
    pub struct Health {
        total: i32,
        current: i32,
    }

    fn init_health () -> Health {
        Health {
            total: 1,
            current: 1,
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
    use std::io;
    pub struct Item {
        name: String,
        enchantment: i32,
        inventory: bool,
        bonus: i32,
        value: i32,
    }
    pub struct Armor {
        total_ac: i32,
        head: Item,
        chest: Item,
        legs: Item,
        feet: Item,
        hands: Item,
    }
    pub struct Equipped {
        weapon: Item,
        off_hand: Item,
        armor: Armor,
        ring1: Item,
        ring2: Item,
        amulet: Item,
        backpack: Inventory,
    }

    impl Equipped{
        pub fn init_equipped (&mut self){
                self.weapon = Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    inventory: false,
                    bonus: 0,
                    value: 0,
                };
                self.off_hand = Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    inventory: false,
                    bonus: 0,
                    value: 0,
                };
                self.armor = Armor {
                    total_ac: 10,
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
                };
                self.ring1 = Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    inventory: false,
                    bonus: 0,
                    value: 0,
                };
                self.ring2 = Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    inventory: false,
                    bonus: 0,
                    value: 0,
                };
                self.amulet = Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    inventory: false,
                    bonus: 0,
                    value: 0,
                };
                self.backpack = Inventory::init_inventory();
            }
           
        fn add_equipped (&mut self, item_type: &str, item: Item) {
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
                _ => (),
            }
        }

        fn find_equipped (&mut self, item_type: &str) -> bool {
            match item_type {
                "Weapon" => {
                    if self.weapon.name == "None" {
                        return false;
                    }
                    return true;
                }
                "Off Hand" => {
                    if self.off_hand.name == "None" {
                        return false;
                    }
                    return true;
                }
                "Armor" => {
                    // Figure this out
                }
                "Ring 1" => {
                    if self.ring1.name == "None" {
                        return false;
                    }
                    return true;
                }
                "Ring 2" => {
                    if self.ring2.name == "None" {
                        return false;
                    }
                    return true;
                }
                "Amulet" => {
                    if self.amulet.name == "None" {
                        return false;
                    }
                    return true;
                }
                _ => {println!("Incorrect Item Type");()},
            }
        }
        
        fn remove_equipped(&mut self, item_type: &str) {
            match item_type {
                "Weapon" => {
                    if self.backpack.add_item(self.weapon) 
                    {
                        self.weapon = Item {
                            name: "None".to_string(),
                            enchantment: 0,
                            inventory: false,
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
                    inventory: false,
                    bonus: 0,
                    value: 0,
                },
                "Armor" => (),
                "Ring 1" => self.ring1 = Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    inventory: false,
                    bonus: 0,
                    value: 0,
                },
                "Ring 2" => self.ring2 = Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    inventory: false,
                    bonus: 0,
                    value: 0,
                },
                "Amulet" => self.amulet = Item {
                    name: "None".to_string(),
                    enchantment: 0,
                    inventory: false,
                    bonus: 0,
                    value: 0,
                },
                _ => (),
            }
        }
    }
    pub struct Inventory {
        max_items: usize,
        items: Vec<Item>,
    }

    impl Inventory {
        pub fn init_inventory (& mut self) {
            self.max_items = 0;
            self.items = Vec::<Item>::new();
        }
        pub fn add_item (&mut self, item: Item) -> bool {
            if self.max_items < self.items.len(){
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
                return self.items[index]
            }
            else {
                println!("Item not found");
                return Item {
                    name: "".to_string(),
                    enchantment: 0,
                    inventory: false,
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