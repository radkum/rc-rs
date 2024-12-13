use std::{
    io::{Read, Write},
    path::Path,
};

use snafu::ResultExt;

use crate::{OpenRcSnafu, ReadRcSnafu, Result};

pub(crate) mod rc_parser;
use rc_parser::{RcParser, Resources};

pub struct ResourceCompiler {
    resources: Resources,
}

impl ResourceCompiler {
    pub fn parse_file(p: impl AsRef<Path>) -> Result<Self> {
        let s = Self::read_from_file(p)?;
        Self::parse(s)
    }

    pub fn parse_stream<R: Read>(reader: R) -> Result<Self> {
        let s = Self::read(reader)?;
        Self::parse(s)
    }

    fn parse(s: String) -> Result<Self> {
        let resources = RcParser::parse_input_string(s.as_str())?;
        Ok(Self { resources })
    }

    fn read_from_file(p: impl AsRef<Path>) -> Result<String> {
        let path = p.as_ref().to_path_buf();
        let file = std::fs::File::open(p).context(OpenRcSnafu { path })?;
        Self::read(file)
    }

    fn read<R: Read>(mut reader: R) -> Result<String> {
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).context(ReadRcSnafu {})?;
        Ok(buffer)
    }

    pub fn write_to_file(&self, p: impl AsRef<Path>) -> Result<()> {
        let path = p.as_ref().to_path_buf();
        let file = std::fs::File::create(p).context(OpenRcSnafu { path })?;
        self.write_to_stream(file)
    }

    fn write_to_stream<W: Write>(&self, mut out: W) -> Result<()> {
        //out.write_all(&resources.serialize()).context(WriteRcSnafu {})?;
        Ok(())
    }
}

pub(crate) trait Serializer {
    fn serialize(&self) -> Vec<u8>;
}
