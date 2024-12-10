use std::io::Write;
use std::io::Read;
use std::path::Path;
use snafu::ResultExt;

use crate::ReadRcSnafu;
use crate::Result;
use crate::OpenRcSnafu;
use crate::WriteRcSnafu;

mod rc_parser;
use rc_parser::RcParser;

pub struct ResourceCompiler {
    file_block: Option<String>,
}

impl ResourceCompiler {
    pub fn new() -> Self {
        Self { file_block: None}
    }

    pub fn parse_file(&mut self, p: impl AsRef<Path>) -> Result<()> {
        let s = self.read_from_file(p)?;
        self.parse(s)
    }

    pub fn parse_stream<R: Read>(&mut self, reader: R) -> Result<()> {
        let s = self.read(reader)?;
        self.parse(s)
    }
    
    fn parse(&mut self, s: String) -> Result<()> {
        //let parser = Parser::new();
        //self.file_block = Some(parser.parse(tokens)?);
        Ok(())
    }

    fn read_from_file(&self, p: impl AsRef<Path>) -> Result<String> {
        let path = p.as_ref().to_path_buf();
        let file = std::fs::File::open(p).context(OpenRcSnafu {path})?;
        self.read(file)
    } 

    fn read<R: Read>(&self, mut reader: R) -> Result<String> {
        let mut buffer= String::new();
        reader.read_to_string(&mut buffer).context(ReadRcSnafu {})?;
        Ok(buffer)
    }

    pub fn write_to_file(&self, p: impl AsRef<Path>) -> Result<()> {
        let path = p.as_ref().to_path_buf();
        let file = std::fs::File::create(p).context(OpenRcSnafu {path})?;
        self.write(file)
    }

    fn write<W: Write>(&self, mut out: W) -> Result<()> {
        let Some(ref file_block) = self.file_block else {
            return Err(crate::RcError::NotParsed{});
        };
        //out.write_all(&file_block.serialize()).context(WriteRcSnafu {})?;
        Ok(())
    }
}

pub(crate) trait Serializer {
    fn serialize(&self) -> Vec<u8>;
}