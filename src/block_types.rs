use serde_derive::Deserialize;
use toml::value::{Table, Array};
use once_cell::sync::OnceCell;
use std::{fs, slice::SliceIndex};

use crate::Direction;

pub static BLOCK_TYPES: OnceCell<Vec<Type>> = OnceCell::new();

#[derive(Deserialize, Debug)]
pub struct Type {
    pub name: String,
    pub id: u16,
    textures: Table,
    pub states: Array
}

impl Type {
    pub fn get_texture(&self, direction: &Direction) -> u16 {
        self.textures.get(&direction.get_string()).unwrap().as_integer().unwrap() as u16
    }
}

pub fn init() {
    let mut blocks: Vec<Type> = Vec::new();
    let block_paths = fs::read_dir("src/assets/blocks/").unwrap();

    for path in block_paths {
        let bytes = fs::read(path.unwrap().path()).expect("Could not read file");
        let block: Type = toml::from_slice(&bytes).unwrap();
        if block.id as usize >= blocks.len() {
            blocks.push(block)
        } else {
            blocks.insert(block.id.into(), block)
        }
    }

    match BLOCK_TYPES.set(blocks) {
        Ok(_) => {}
        Err(types) => panic!("Failed to initialize block types: {:?}", types)
    }
}

pub fn get(id: u16) -> &'static Type {
    BLOCK_TYPES.get().expect("You tried to get a block type before the block files have been deserialized!").get(id as usize).expect("Block type does not exist!")
}