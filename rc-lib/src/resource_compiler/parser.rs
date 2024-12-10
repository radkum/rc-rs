use crate::Result;
use super::FileBlock;
use super::Tokens;
pub(super) struct Parser {

}

impl Parser {
    pub(crate) fn new() -> Self {
        Self{}
    }

    pub(crate) fn parse(&self, _tokens: Tokens) -> Result<FileBlock> {
        todo!()
    }
}