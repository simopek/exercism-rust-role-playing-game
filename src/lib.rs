// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health > 0 {
            true => None,
            false => match self.level {
                level if level < 10 => Some(Player { health: 100, level: self.level, mana: None }),
                _ => Some(Player { health: 100, level: self.level, mana: Some(100) })
            }
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {

        match self.mana {
            None => {
                self.health = if self.health >= mana_cost {self.health - mana_cost} else {0};
                0
            },
            Some(x) if x < mana_cost => 0,
            Some(x) => {
                self.mana = Some(x - mana_cost);
                mana_cost * 2
            }
        }
    }
}
