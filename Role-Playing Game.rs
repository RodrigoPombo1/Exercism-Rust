pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            let mut player_mana = None;
            if self.level >= 10 { 
                player_mana = Some(100);
            }
            let player = Player { health: 100, mana: player_mana, level: self.level };
            Some(player)
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.mana == None {
            if self.health > mana_cost {
                self.health -= mana_cost;
            } else {
                self.health = 0;
            }
            return 0;
        }
        if self.mana < Some(mana_cost) {
            return 0;
        }
        self.mana = Some(self.mana.unwrap() - mana_cost);
        return mana_cost * 2;
    }
}
