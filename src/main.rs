mod tatoeba;
mod kanjidic2;

use std::process;
use tatoeba::filter_language;

use clap::{Arg, App, SubCommand};

fn main() {

    let matches = App::new("JLT command line tool")
        .version("1.0")
        .author("André Twupack")
        .subcommand(SubCommand::with_name("tatoeba-filter")
            .about("Filter a Tatoeba sentences file by language code.")
            .arg(Arg::with_name("input")
                .short("i")
                .takes_value(true)
                .required(true)
            )
            .arg(Arg::with_name("output")
                .short("o")
                .takes_value(true)
                .required(true)
            )
            .arg(Arg::with_name("language")
                .short("l")
                .takes_value(true)
                .required(true)
            )
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("tatoeba-áfilter") {
        let input = matches.value_of("input").unwrap();
        let output = matches.value_of("output").unwrap();
        let language = matches.value_of("language").unwrap();

        if let Err(err) = filter_language(input, output, language) {
            println!("error running filter: {}", err);
            process::exit(1);
        }
    }
}
