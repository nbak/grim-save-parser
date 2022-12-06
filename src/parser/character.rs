use std::cell::RefMut;
use std::{cell::RefCell, io::Read};

use crate::util::{CustomError, Result};

use super::model::CharacterFile;
use super::parser::{Block, Parser};
use super::Readable;
use super::Reader;

pub struct CharacterReader<T: Read> {
    source: RefCell<T>,
    key: u32,
    table: [u32; 256],
    cur_pos: u64,
    blocks: Vec<Block>,
}

impl<T: Read> Reader for CharacterReader<T> {
    type Item = CharacterFile;
    type Source = T;

    fn new(source: T) -> Self {
        CharacterReader {
            source: RefCell::new(source),
            key: 0,
            table: [0; 256],
            cur_pos: 0,
            blocks: Vec::new(),
        }
    }
    fn read(&mut self) -> Result<CharacterFile> {
        self.read_key()?;

        CharacterFile::read_from(self)
    }
}

impl<T: Read> Parser for CharacterReader<T> {
    fn get_source(&self) -> RefMut<dyn Read> {
        self.source.borrow_mut()
    }
    fn get_key(&self) -> u32 {
        self.key
    }
    fn get_pos(&self) -> u64 {
        self.cur_pos
    }
    fn inc_pos(&mut self, v: u64) {
        self.cur_pos = self.cur_pos + v;
    }
    fn set_key(&mut self, key: u32) {
        self.key = key;
    }
    fn pop_block(&mut self) -> Result<Block> {
        self.blocks
            .pop()
            .ok_or(CustomError::new("Pop from empty stack".to_owned()))
    }
    fn push_block(&mut self, b: Block) {
        self.blocks.push(b);
    }
    fn get_table_elem(&self, index: usize) -> u32 {
        self.table[index]
    }
    fn set_table_elem(&mut self, index: usize, elem: u32) {
        self.table[index] = elem;
    }
}
