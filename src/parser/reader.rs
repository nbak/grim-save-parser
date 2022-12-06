use std::io::Read;

use crate::util::Result;

use super::Readable;

pub trait Reader {
    type Source: Read;
    type Item: Readable;

    fn new(source: Self::Source) -> Self where Self: Sized;
    fn read(&mut self) -> Result<Self::Item>;
}
