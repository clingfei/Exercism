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
        if self.health == 0 {
            if self.level >= 10 {
                return Some(Player{
                    health: 100, mana: Some(100), level: self.level
                });
            } else {
                return Some(Player{
                    health: 100, mana: None, level: self.level
                });
            }
        } else {
            return None;
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if let Some(x) = self.mana {
            if x > mana_cost {
                self.mana = Some(x - mana_cost);
                return mana_cost * 2;
            } else {
                return 0;
            }
        } else {
            if self.health > mana_cost {
                self.health -= mana_cost;
            } else {
                self.health = 0;
            }
            return 0;
        }
    }
}
