use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use snafu::ResultExt;

use crate::{PestParsingSnafu, Result};

mod resources;
pub(crate) use resources::Resources;

#[derive(Parser, Default)]
#[grammar = "rc_compiler/rc.pest"]
pub(super) struct RcParser {
    resources: Resources,
}

impl RcParser {
    pub(crate) fn parse_string(input: &str) -> Result<Resources> {
        let mut tokens = Self::parse(Rule::res, input)
            .context(PestParsingSnafu { function_name: "parse_string()".to_string() })?;
        let res = tokens.next().unwrap();

        if res.as_rule() != Rule::res {
            panic!("expected rule res");
        }

        let this = Self::default();
        this.parse_res(res)?;
        Ok(this.resources)
    }

    fn parse_res(&self, res: Pair<Rule>) -> Result<()> {
        todo!()
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
