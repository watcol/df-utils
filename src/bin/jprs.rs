//! JSON(JSON5) Parser
use clap::Clap;
use df_utils::*;
use std::path::PathBuf;

/// Options
#[derive(Clone, Debug, Clap)]
#[clap(
    name = "jprs",
    version = clap::crate_version!(),
    author = clap::crate_authors!(),
    about = "Simple CLI JSON(JSON5) Parser"
)]
struct Opts {
    #[clap(name = "INPUT", about = "The input JSON file.")]
    input: Option<PathBuf>,
    #[clap(short = 'o', long = "output", about = "The output file.")]
    output: Option<PathBuf>,
    #[clap(
        short = 'r',
        long = "root",
        about = "The root indicator.",
        default_value = "$"
    )]
    root: String,
    #[clap(
        short = 'd',
        long = "delimiter",
        about = "The delimiter for the path.",
        default_value = "."
    )]
    delimiter: String,
    #[clap(
        short = 'e',
        long = "equal",
        about = "The equal symbol.",
        default_value = " = "
    )]
    equal: String,
    #[clap(short = '5', long = "json5", about = "Enable JSON5 expanded syntax.")]
    json5: bool,
}

fn main() -> std::io::Result<()> {
    let opts = Opts::parse();

    let value = if opts.json5 {
        parser::Json5Parser.read_path(opts.input)
    } else {
        parser::JsonParser.read_path(opts.input)
    }
    .unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(1);
    });

    generator::LineGenerator::new()
        .root(opts.root)
        .delimiter(opts.delimiter)
        .equal(opts.equal)
        .write_path(opts.output, &value)
}
