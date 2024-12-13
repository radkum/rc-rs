mod cli;
use clap::Parser;
use rc_lib::ResourceCompiler;
fn main() -> anyhow::Result<()> {
    //tmp
    let input_str = r#"#include <winver.h>
#include <windows.h>

desk1   ICON "desk.ico"

VS_VERSION_INFO VERSIONINFO
    FILEVERSION 1,2,3,4
    PRODUCTVERSION 5,6,7,8
    FILEFLAGSMASK 0x3fL
BEGIN
    BLOCK "StringFileInfo"
    BEGIN
        BLOCK "040904b0"
        BEGIN
            VALUE "CompanyName", "Datto, Inc"
            VALUE "FileDescription", "Datto EDR Agent"
            VALUE "FileVersion", "0.0.0.0"
            VALUE "LegalCopyright", "Copyright (C) 2015-YYYY"
            VALUE "ProductName", "Datto EDR Agent"
            VALUE "ProductVersion", "0.0.0.0"
        END
    END
    BLOCK "VarFileInfo"
    BEGIN
        VALUE "Translation", 0x409, 1200
    END
END

11      ICON "custom.ico"
11      ICON "custom.ico""#;

    let mut rc_compiler = ResourceCompiler::parse_stream(input_str.as_bytes())?;
    rc_compiler.write_to_file("resource.rc".to_string())?;

    return Ok(());

    let args = cli::ConfigCli::parse();

    log::debug!("{args:?}");

    let mut rc_compiler = ResourceCompiler::parse_file(args.input_file)?;

    let output_file = args.output_file.unwrap_or("resource.rc".to_string());
    rc_compiler.write_to_file(output_file)?;

    Ok(())
}
