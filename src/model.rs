use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CatState {
    affection: u8, // 愛情度（0-100%）
    energy: u8, // エネルギー（0-100%）
    stage: u8, // 0: Baby, 1: Teen, 2: Adult
}

impl Default for CatState {
    fn default() -> Self {
        Self {
            affection: 20,
            energy: 20,
            stage: 0,
        }
    }
}