//! JSON Generator
use clap::Clap;
use df_utils::{
    io::{Input, Output},
    Generator, MinJsonGenerator, PrettyJsonGenerator, PrintConfig, Value,
};
use std::io::Read;
use std::path::PathBuf;

/// Options
#[derive(Clone, Debug, Clap)]
#[clap(
    name = "jgen",
    version = clap::crate_version!(),
    author = clap::crate_authors!(),
    about = "Simple CLI JSON Generator"
)]
struct Opts {
    #[clap(name = "INPUT", about = "The input file.")]
    input: Option<PathBuf>,
    #[clap(short = 'o', long = "output", about = "The output JSON file.")]
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
    #[clap(short = 'm', long = "minify", about = "Minify JSON output.")]
    minify: bool,
}

fn main() -> std::io::Result<()> {
    let opts = Opts::parse();

    let mut s = String::new();
    Input::from_path(opts.input)?.read_to_string(&mut s)?;

    let value = Value::parse(
        &s,
        PrintConfig::new()
            .root(opts.root)
            .delimiter(opts.delimiter)
            .equal(opts.equal),
    )
    .unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(1);
    });

    let mut output = Output::from_path(opts.output)?;
    if opts.minify {
        MinJsonGenerator.generate(&mut output, &value)?;
    } else {
        PrettyJsonGenerator.generate(&mut output, &value)?;
    }

    Ok(())
}
