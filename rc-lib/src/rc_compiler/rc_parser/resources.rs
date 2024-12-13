use std::str::FromStr;

use smart_default::SmartDefault;

use crate::rc_compiler::rc_parser::{version_info::VersionInfoRes, RcParser};

#[derive(Default, Debug)]
pub(crate) struct Resources {
    includes: Vec<String>,
    resources: Vec<Resource>,
}

impl Resources {
    pub(crate) fn add_include(&mut self, include: String) {
        self.includes.push(include);
    }

    pub(crate) fn add_resource(&mut self, resource: Resource) {
        self.resources.push(resource);
    }
}

#[derive(SmartDefault, Debug)]
pub(crate) enum Resource {
    #[default]
    Icon(IconRes),
    VersionInfo(VersionInfoRes),
}

#[derive(SmartDefault, Debug)]
pub(crate) enum NameId {
    #[default]
    Int(u16),
    Str(String),
}

#[derive(Default, Debug)]
pub(crate) struct IconRes {
    nameId: NameId,
    filename: String,
}

impl IconRes {
    pub fn new(name_id: NameId, filename: String) -> Self {
        Self { nameId: name_id, filename }
    }
}

impl FromStr for NameId {
    type Err = super::PestError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Ok(if let Ok(int) = RcParser::hex_to_int(str) {
            NameId::Int(int as u16)
        } else {
            NameId::Str(str.to_string())
        })
    }
}
