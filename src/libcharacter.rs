use serde::{Serialize, Deserialize};
#[derive(Default, Copy, Clone, Serialize, Deserialize)]
pub struct Character {
    pub stats: [u8; 6],
}

impl Character {
    pub fn init(&mut self, strength: u8, dex: u8, con: u8, wis: u8, intel: u8, cha: u8) {
        self.stats[0] = strength;
        self.stats[1] = dex;
        self.stats[2] = con;
        self.stats[3] = wis;
        self.stats[4] = intel;
        self.stats[5] = cha;
    }
}


