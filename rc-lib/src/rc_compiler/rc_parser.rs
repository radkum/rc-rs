use crate::Result;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "rc_compiler/rc.pest"]
pub(super) struct RcParser {

}

impl RcParser {
    pub(crate) fn new() -> Self {
        Self{}
    }

    pub(crate) fn parse_resource(input: &str) -> Result<()> {
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