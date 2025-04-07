use clap::Parser;
use trgt::{
    utils::{handle_error_and_exit, Result},
    hmm::builder
};


#[derive(Parser)]
struct AltCli {
    #[arg(short('i'), long("input"))]
    input: String,
    #[arg(short('o'), long("output"))]
    output: String,
}

fn alt_runner() -> Result<()> {
    let cli = AltCli::parse();
    builder::parse_SVA_repeat_file(&cli.input, &cli.output);
    Ok(())
}

fn main() {
    if let Err(e) = alt_runner() {
        handle_error_and_exit(e);
    }
}
