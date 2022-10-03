use clap::{
    crate_authors, crate_description, crate_name, crate_version, AppSettings, Command, Parser,
};
use std::path::PathBuf;

/*
#[derive(Debug)]
struct Args {
    verbosity: usize,
}
*/

/*
let raw = clap_lex::RawArgs::new([
    "bin",
    "--output-file",
    "out.txt",
    "add",
    "file1.txt",
    "--name",
    "Dub, Blu-Ray",
]);
let mut cursor = raw.cursor();

raw.next(&mut cursor); // Skip the bin

while let Some(arg) = raw.next(&mut cursor) {
    println!(
        "{:?}, is_empty={}, is_escape={}, is_long={}, is_number={}, is_short={}, is_stdio={}",
        arg,
        arg.is_empty(),
        arg.is_escape(),
        arg.is_long(),
        arg.is_number(),
        arg.is_short(),
        arg.is_stdio(),
    );
}
*/

#[derive(Parser)]
#[clap(name = crate_name!())]
#[clap(author = crate_authors!("\n"))]
#[clap(version = crate_version!())]
#[clap(about = crate_description!(), long_about = None)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
struct Cli {
    /// Overwrites the existing output .mp4 file if there is one
    #[clap(long)]
    overwrite: bool,

    /// Specifies the ISO base media file format brand in the format
    #[clap(long, value_name = "arg")]
    mpeg4_brand: Option<String>,

    /// Output .mp4 file name
    #[clap(long, value_name = "FILE", parse(from_os_str))]
    output_file: PathBuf,

    #[clap(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
enum Action {
    Add(Add),
}

#[derive(Parser, Debug)]
struct Add {
    /// Input track file name
    #[clap(value_name = "FILE", parse(from_os_str))]
    input: PathBuf,

    /// Media name, e.g. 'Dub, Blu-ray'
    #[clap(
        long = "name",
        value_name = "name",
        multiple_values = false,
        use_value_delimiter = false
    )]
    media_name: Option<String>,
}

fn main() {
    let cli = Cli::parse_from([
        "clap-usage",
        "--overwrite",
        "--mpeg4-brand",
        "mp42",
        "--output-file",
        "movie.mp4",
        "add",
        "video.hevc",
        "--name",
        "some movie name",
    ]);
    /*
    let s = "Movie.HDR.DV81.hevc,name=MP4muxer\\, v2.1.0 (based on https://github.com/DolbyLaboratories/dlb_mp4base)";

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .quoting(false)
        .escape(Some(b'\\'))
        .from_reader(s.as_bytes());
    s.spl

    for record in reader.records() {
        println!("{:?}", record.unwrap());
    }
    */

    // println!("{:?}", shlex::split(s).unwrap());
}
