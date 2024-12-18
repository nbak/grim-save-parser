use std::{
    fs::File,
    io::{stdin, Read},
    path::PathBuf,
};

use clap::{
    builder::{PossibleValuesParser, TypedValueParser as _},
    Parser, ValueHint,
};

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(
        short,
        long,
        default_value_t = String::from("character"),
        value_parser = PossibleValuesParser::new(["character", "formulas", "stash"])
            .map(|s| s.to_owned())
        )]
    entity_type: String,

    #[arg(short, long, value_hint = ValueHint::FilePath)]
    filepath: Option<PathBuf>,
}

pub struct Input {
    pub entity_type: String,
    pub source: Vec<u8>,
}

pub fn get_data() -> Input {
    let Cli {
        entity_type,
        filepath,
    } = Cli::parse();

    let source = filepath.map_or_else(
        || {
            let mut buffer = Vec::new();
            stdin().read_to_end(&mut buffer).unwrap();
            buffer
        },
        |path| {
            let mut buffer = Vec::new();
            File::open(path)
                .expect("Cannot open file")
                .read_to_end(&mut buffer)
                .unwrap();
            buffer
        },
    );

    Input {
        entity_type,
        source,
    }
}
