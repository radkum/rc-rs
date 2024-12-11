use smart_default::SmartDefault;

#[derive(Default, Debug)]
pub(crate) struct Resources {
    includes: Vec<String>,
    resources: Vec<Resource>,
}

#[derive(SmartDefault, Debug)]
enum Resource {
    #[default]
    Icon(IconRes),
    VersionInfo(VersionInfoRes),
}

#[derive(SmartDefault, Debug)]
enum NameId {
    #[default]
    Int(u16),
    Str(String),
}

#[derive(Default, Debug)]
struct IconRes {
    nameId: NameId,
    filename: String,
}

#[derive(Default, Debug)]
struct VersionInfoRes {
    info: VersionInfo,
    blocks: Vec<Block>,
}

#[derive(Default, Debug)]
struct VersionId(String);

#[derive(Default, Debug)]
struct BinaryVersion(u32, u32);

#[derive(Default, Debug)]
struct FileFlags(u16);

#[derive(Default, Debug)]
struct VersionInfo {
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
