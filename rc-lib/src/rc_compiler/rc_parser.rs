use std::num::ParseIntError;

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use snafu::ResultExt;

mod resources;
pub use resources::*;

mod version_info;

use snafu::Snafu;

use crate::rc_compiler::rc_parser::{
    version_info::VersionInfoRes,
    PestError::{MissingQuotation, UnexpectedRule},
};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum PestError {
    #[snafu(display("Failed to parse in function: {}. Error: {}", function_name, source))]
    Parsing { function_name: String, source: pest::error::Error<Rule> },

    #[snafu(display(
        "Unexpected token. Expected rule: {:?}, got rule: {:?}, token: {}",
        expected_rule,
        token_rule,
        token_name
    ))]
    UnexpectedRule { expected_rule: Rule, token_rule: Rule, token_name: String },

    #[snafu(display("Missing quotation. Input str: {}", token))]
    MissingQuotation { token: String },

    #[snafu(display("Failed convert hex str to int: {}. Error: {}", token, source))]
    FromHexToInt { token: String, source: ParseIntError },

    #[snafu(display("Failed convert str to int: {}. Error: {}", token, source))]
    FromStrToInt { token: String, source: ParseIntError },
}
pub type Result<T> = std::result::Result<T, PestError>;

#[derive(Parser, Default)]
#[grammar = "rc_compiler/rc.pest"]
pub(super) struct RcParser {
    resources: Resources,
}

impl RcParser {
    pub(crate) fn parse_input_string(input: &str) -> Result<Resources> {
        let mut tokens = Self::parse(Rule::res, input)
            .context(ParsingSnafu { function_name: "parse_string()".to_string() })?;
        let res = tokens.next().unwrap();

        if res.as_rule() != Rule::res {
            panic!("expected rule res");
        }

        let mut this = Self::default();
        this.parse_res(res)?;
        Ok(this.resources)
    }

    fn parse_res(&mut self, token: Pair<Rule>) -> Result<()> {
        Self::check_rule(&token, Rule::res)?;

        let mut iter = token.into_inner().into_iter();
        let includes = iter.next().unwrap();
        self.parse_includes(includes)?;

        //let entry = iter.next().unwrap();
        //self.parse_resource_entry(entry)?;

        for entry in iter {
            self.parse_resource_entry(entry)?;
        }

        Ok(())
    }

    fn parse_includes(&mut self, token: Pair<Rule>) -> Result<()> {
        Self::check_rule(&token, Rule::includes)?;

        for include in token.into_inner().into_iter() {
            //let inc = Self::remove_quotation(include.into_inner().as_str())?;
            let inc = include.into_inner().as_str();
            //println!("{}", inc);
            self.resources.add_include(inc.to_string());
        }
        Ok(())
    }

    fn parse_resource_entry(&mut self, entry: Pair<Rule>) -> Result<()> {
        let resource = match entry.as_rule() {
            Rule::icon => Resource::Icon(Self::parse_icon(entry)?),
            Rule::version_info => Resource::VersionInfo(VersionInfoRes::parse(entry)?),
            _ => {
                log::error!("There should be icon or version_info");
                return Err(UnexpectedRule {
                    expected_rule: Rule::res,
                    token_rule: entry.as_rule(),
                    token_name: entry.to_string(),
                });
            },
        };
        println!("{resource:?}");
        self.resources.add_resource(resource);
        Ok(())
    }

    fn parse_icon(token: Pair<Rule>) -> Result<IconRes> {
        Self::check_rule(&token, Rule::icon)?;
        let mut tokens = token.into_inner().into_iter();
        let name_id = tokens.next().unwrap();
        let name_id = name_id.as_str().parse()?;

        let file_name = tokens.next().unwrap();
        let file_name = file_name.as_str();
        let file_name = Self::remove_quotation(file_name)?;

        Ok(IconRes::new(name_id, file_name))
    }

    fn check_rule(token: &Pair<Rule>, expected_rule: Rule) -> Result<()> {
        if token.as_rule() != expected_rule {
            return Err(PestError::UnexpectedRule {
                expected_rule,
                token_rule: token.as_rule(),
                token_name: token.to_string(),
            });
        }

        Ok(())
    }

    fn remove_quotation(s: &str) -> Result<String> {
        let str = s.to_string();
        let Some(str) = str.strip_prefix("\"") else {
            return Err(MissingQuotation { token: s.to_string() });
        };

        let Some(str) = str.strip_suffix("\"") else {
            return Err(MissingQuotation { token: s.to_string() });
        };

        Ok(str.to_string())
    }

    pub fn hex_to_int(hex_str: &str) -> Result<u32> {
        let final_int = if let Some(int_str) = hex_str.strip_prefix("0x") {
            u32::from_str_radix(int_str, 16).context(FromHexToIntSnafu { token: hex_str })?
        } else if let Some(int_str) = hex_str.strip_prefix("0X") {
            u32::from_str_radix(int_str, 16).context(FromHexToIntSnafu { token: hex_str })?
        } else {
            u32::from_str_radix(hex_str, 10).context(FromHexToIntSnafu { token: hex_str })?
        };
        Ok(final_int)
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;

    use crate::rc_compiler::rc_parser::*;

    #[test]
    fn parse_icon() {
        let icon_res = r#"desk1   ICON "desk.ico""#;
        let mut tokens = RcParser::parse(Rule::res, icon_res).unwrap();
        let mut res = tokens.clone().next().unwrap();

        let mut iter = res.clone().into_inner().into_iter();
        let includes = iter.next().unwrap();
        let icon = iter.next().unwrap();

        let mut icon_iter = icon.into_inner().into_iter();
        let icon_name = icon_iter.next().unwrap().as_str();
        let icon_filename = icon_iter.next().unwrap().as_str();

        assert_eq!(icon_name, "desk1");
        assert_eq!(icon_filename, "\"desk.ico\"");
    }
}
