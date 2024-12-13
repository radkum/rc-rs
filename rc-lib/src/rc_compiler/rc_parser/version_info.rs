use pest::iterators::Pair;
use snafu::ResultExt;

use super::{
    resources::{Resource, *},
    FromHexToIntSnafu, RcParser, Result, Rule,
};
use crate::rc_compiler::rc_parser::FromStrToIntSnafu;
#[derive(Default, Debug)]
pub(crate) struct VersionInfoRes {
    info: VersionHeader,
    blocks: Vec<Block>,
}

#[derive(Default, Debug)]
pub(crate) struct VersionId(String);

#[derive(Default, Debug)]
pub(crate) struct BinaryVersion(u16, u16, u16, u16);

#[derive(Default, Debug)]
pub(crate) struct FileFlags(u16);

#[derive(Default, Debug)]
pub(crate) struct VersionHeader {
    version_id: VersionId,
    file_version: Option<BinaryVersion>,
    product_version: Option<BinaryVersion>,
    fileflags_mask: Option<FileFlags>,
}

#[derive(Default, Debug)]
struct Block {
    name: String,
    string_block: Option<StringBlock>,
    var_file_block: Option<VarFileBlock>,
}

#[derive(Default, Debug)]
struct StringBlock {
    name: String,
    values: Vec<(String, String)>,
}

#[derive(Default, Debug)]
struct VarFileBlock {
    name: String,
    values: Vec<(String, u16, u16)>,
}

impl VersionInfoRes {
    pub(crate) fn parse(token: Pair<Rule>) -> Result<Self> {
        RcParser::check_rule(&token, Rule::version_info)?;

        let mut tokens = token.into_inner().into_iter();

        let version_header_token = tokens.next().unwrap();

        let mut version_info = Self::default();
        version_info.parse_version_header(version_header_token)?;

        for block in tokens {
            version_info.parse_block(block)?;
        }

        Ok(version_info)
    }

    pub(crate) fn parse_version_header(&mut self, token: Pair<Rule>) -> Result<()> {
        RcParser::check_rule(&token, Rule::version_info_header)?;
        let mut tokens = token.into_inner().into_iter();
        let version_info_tag_token = tokens.next().unwrap();
        self.info.version_id = Self::parse_version_info_tag(version_info_tag_token)?;

        for entry in tokens {
            self.parse_version_header_entry(entry)?;
        }
        Ok(())
    }

    pub(crate) fn parse_version_info_tag(token: Pair<Rule>) -> Result<VersionId> {
        RcParser::check_rule(&token, Rule::version_info_tag)?;
        let version_info_tag = token.into_inner().next().unwrap().as_str();
        println!("{}", version_info_tag);
        Ok(VersionId(version_info_tag.to_string()))
    }

    pub(crate) fn parse_version_header_entry(&mut self, token: Pair<Rule>) -> Result<()> {
        match token.as_rule() {
            Rule::file_version => {
                let file_version = Self::parse_binary_version(token)?;
                self.info.file_version = Some(file_version);
            },
            Rule::product_version => {
                let product_version = Self::parse_binary_version(token)?;
                self.info.product_version = Some(product_version);
            },
            Rule::file_flags_mask => {
                let file_flags_mask = Self::parse_hex_long(token)?;
                self.info.fileflags_mask = Some(file_flags_mask);
            },
            _ => todo!(),
        }
        Ok(())
    }

    pub(crate) fn parse_block(&mut self, token: Pair<Rule>) -> Result<()> {
        Ok(())
    }

    fn parse_binary_version(token: Pair<Rule>) -> Result<BinaryVersion> {
        let mut tokens = token.into_inner().into_iter();
        let version_dword = tokens.next().unwrap();
        let mut dword_iter = version_dword.into_inner().into_iter();

        let hiword_dw1 = dword_iter.next().unwrap().as_str();
        let loword_dw1 = dword_iter.next().unwrap().as_str();
        let hiword_dw2 = dword_iter.next().unwrap().as_str();
        let loword_dw2 = dword_iter.next().unwrap().as_str();

        let hiword_dw1_int: u16 =
            hiword_dw1.parse().context(FromStrToIntSnafu { token: hiword_dw1 })?;
        let loword_dw1_int: u16 =
            loword_dw1.parse().context(FromStrToIntSnafu { token: loword_dw1 })?;
        let hiword_dw2_int: u16 =
            hiword_dw2.parse().context(FromStrToIntSnafu { token: hiword_dw2 })?;
        let loword_dw2_int: u16 =
            loword_dw2.parse().context(FromStrToIntSnafu { token: loword_dw2 })?;
        Ok(BinaryVersion(hiword_dw1_int, loword_dw1_int, hiword_dw2_int, loword_dw2_int))
    }

    fn parse_hex_long(token: Pair<Rule>) -> Result<FileFlags> {
        let mut tokens = token.into_inner().into_iter();
        let hex = tokens.next().unwrap();
        let hex_str = hex.into_inner().as_str();
        let final_int = RcParser::hex_to_int(hex_str)?;
        Ok(FileFlags(final_int as u16))
    }
}
