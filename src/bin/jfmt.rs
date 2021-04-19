//! JSON(JSON5) Formatter
use clap::Clap;
use df_utils::*;
use std::io::Read;
use std::path::PathBuf;

/// Options
#[derive(Clone, Debug, Clap)]
#[clap(
    name = "jfmt",
    version = clap::crate_version!(),
    author = clap::crate_authors!(),
    about = "Simple CLI JSON(JSON5) Formatter"
)]
struct Opts {
    #[clap(name = "INPUT", about = "The input JSON file.")]
    input: Option<PathBuf>,
    #[clap(short = 'o', long = "output", about = "The output file.")]
    output: Option<PathBuf>,
    #[clap(short = '5', long = "json5", about = "Enable JSON5 expanded syntax.")]
    json5: bool,
    #[clap(short = 'm', long = "minify", about = "Minify JSON output.")]
    minify: bool,
    #[clap(
        short = 'i',
        long = "indent",
        about = "The indent size.",
        default_value = "4",
        conflicts_with = "minify"
    )]
    indent: usize,
}

fn main() -> std::io::Result<()> {
    let opts = Opts::parse();
    let mut s = String::new();
    io::Input::from_path(opts.input)?.read_to_string(&mut s)?;
    let value = if opts.json5 {
        parser::Json5Parser.parse(&s)
    } else {
        parser::JsonParser.parse(&s)
    }
    .unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(1);
    });

    let mut output = io::Output::from_path(opts.output)?;

    if opts.minify {
        generator::MinJsonGenerator.generate(&mut output, &value)?;
    } else {
        generator::PrettyJsonGenerator::new()
            .indent(opts.indent)
            .generate(&mut output, &value)?;
    }
    Ok(())
}
