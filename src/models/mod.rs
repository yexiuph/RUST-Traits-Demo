pub struct Player {
    pub player_name: String,
    pub health: i32,
    pub mana: i32,
    pub damage: i32,
    pub range: i32
}

pub trait Character {
    fn get_player_name(&self) -> &str;
    fn get_hp(&self) -> i32;
    fn get_mp(&self) -> i32;
    fn get_range(&self) -> i32;
    fn attack(&self) -> i32;
    fn damage_player(&mut self, damage:i32);
}

impl Character for Player {
    fn get_player_name(&self) -> &str {
        &self.player_name
    }

    fn damage_player(&mut self, damage: i32) {
        self.health = self.health - damage;
        println!("{} Health: {}",self.get_player_name(), self.health);
    }

    fn get_hp(&self) -> i32 {
        self.health
    }

    fn get_mp(&self) -> i32 {
        self.mana
    }

    fn get_range(&self) -> i32 {
        self.range
    }

    fn attack(&self) -> i32 {
        self.damage
    }
}
