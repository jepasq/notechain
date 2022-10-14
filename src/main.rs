use serde::{Serialize, Deserialize};

use chrono::Utc;

const GENHASH:
    &str = "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}

#[derive(Debug)]
pub struct App {
    pub blocks: Vec<Block>,
}

impl App {
    fn new() -> Self {
        Self { blocks: vec![] }
    }

    fn genesis(&mut self) {
        let genesis_block = Block {
            id: 0,
            timestamp: Utc::now().timestamp(),
            previous_hash: String::from("genesis"),
            data: String::from("genesis!"),
            nonce: 2836,
            hash: GENHASH.to_string(),
        };
        self.blocks.push(genesis_block);
    }
}

fn main() {
    let mut a = App::new();
    a.genesis();
    println!("App is {:?}!", a);
}
