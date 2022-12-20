use serde_derive::Deserialize;
use toml::value::{Table, Array};
use once_cell::sync::OnceCell;
use std::fs;

use crate::direction::Direction;

pub static BLOCK_TYPES: OnceCell<Vec<Option<Type>>> = OnceCell::new();

#[derive(Deserialize, Debug, Clone)]
pub struct Type {
    pub name: String,
    pub id: u32,
    textures: Table,
    pub states: Array
}

impl Type {
    pub fn get_texture(&self, direction: Direction) -> u32 {
        self.textures.get(&direction.get_string()).unwrap().as_integer().unwrap() as u32
    }
}

pub fn init() {
    let mut blocks: Vec<Option<Type>> = vec![None; 1000];
    let block_paths = fs::read_dir("src/assets/blocks/").unwrap();

    for path in block_paths {
        let bytes = fs::read(path.unwrap().path()).expect("Could not read file");
        let block: Type = toml::from_slice(&bytes).unwrap();
        let id = block.clone().id;
        
        blocks[id as usize] = Some(block);
    }

    match BLOCK_TYPES.set(blocks) {
        Ok(_) => {}
        Err(types) => panic!("Failed to initialize block types: {:?}", types)
    }
}

pub fn get(id: u32) -> &'static Type {
    BLOCK_TYPES.get().expect("You tried to get a block type before the block files have been deserialized!").get(id as usize).expect("Block type does not exist!").as_ref().expect("Out of bounds!")
}