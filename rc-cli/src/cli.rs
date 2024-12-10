use clap::Parser;

#[derive(Parser, Debug)]
pub struct ConfigCli {
    #[arg(long = "fo")]
    pub output_file: Option<String>,
    #[arg(long = "cd")]
    pub cd: Option<String>,
    #[arg(required(true))]
    pub input_file: String,
}