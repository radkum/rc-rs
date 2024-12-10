use crate::Result;

pub(crate) struct Token {

}

pub(crate) type Tokens = Vec<Token>;

pub(crate) struct Tokenizer {

}

impl Tokenizer {
    pub(crate) fn new()->Self { Self {}}

    pub(crate) fn tokenize(&self, _s: String) -> Result<Tokens> {
        todo!();
    }
}