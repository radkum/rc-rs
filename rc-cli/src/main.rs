mod cli;
use clap::Parser;
use rc_lib::ResourceCompiler;
fn main() -> anyhow::Result<()> {
    let args = cli::ConfigCli::parse();

    log::debug!("{args:?}");

    let mut rc_compiler = ResourceCompiler::parse_file(args.input_file)?;

    let output_file = args.output_file.unwrap_or("resource.rc".to_string());
    rc_compiler.write_to_file(output_file)?;

    Ok(())
}
