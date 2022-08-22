pub struct MonsterInfo {
    health: health::Health,
    stats: super::stats::Stats,
    inventory: super::inventory::Inventory,
    equipped: super::inventory::Equipped,
}

pub fn get_monster_stat (monster: &MonsterInfo, stat: &str) -> i32 {
    super::stats::get_stat(&monster.stats, stat)
}

pub fn create_monster (monstertype: &str) -> MonsterInfo {
    match(monstertype)
    {
        "Orc" => orc::create_orc(),
        "Goblin" => create_goblin(),
        _ => create_villager()
    }
}


mod orc;