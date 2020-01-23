use crate::commands::command::CommandExample;
use crate::examples::*;
use crate::query::fuzzy_search::fuzzy_search;
use crate::utils::display::{display, DisplayFormat};

use clap::{App, Arg};

pub fn run() {
    let matches = App::new("Command Example")
        .version("0.1.0")
        .author("Blas Rodriguez Irizar. <rodrigblas@gmail.com>")
        .about("Search for command examples directly on your command line")
        .arg(
            Arg::with_name("query")
                .short("q")
                .long("query")
                .value_name("QUERY")
                .help("Query a command (fuzzy match)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("display_format")
                .short("d")
                .long("display-format")
                .value_name("display_format")
                .possible_values(&DisplayFormat::variants())
                .case_insensitive(true)
                .help("Select a display format: ")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("COMMAND_NAME")
                .help("Sets the command name to find an example")
                .required(false)
                .index(1),
        )
        .get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let command_name = matches.value_of("COMMAND_NAME");
    let query_fuzzy = matches.value_of("query");
    let display_format = match matches.value_of("display_format").unwrap_or("ascii") {
        "ascii" => DisplayFormat::ASCII,
        "json" => DisplayFormat::JSON,
        _ => DisplayFormat::ASCII,
    };
    match (command_name, query_fuzzy) {
        (Some(cmd_opt), Some(fuzzy_query)) => find_examples_fuzzy(fuzzy_query, Some(cmd_opt), &display_format),
        (None, Some(fuzzy_query)) => find_examples_fuzzy(fuzzy_query, None, &display_format),
        (Some(cmd), _) => find_examples(cmd, &display_format),
        _ => eprintln!("Unrecognized command. Run command_examples --help for more information."),
    }
}

fn find_examples(command_name: &str, display_format: &DisplayFormat) {
    match command_name {
        "find" => find::examples(&display_format),
        "grep" => grep::examples(&display_format),
        "tar" => tar::examples(&display_format),
        _ => println!("{}", format!("No command examples for {}.", command_name)),
    }
}

fn find_examples_fuzzy(query: &str, command_name: Option<&str>, display_format: &DisplayFormat) {
    fuzzy_search(query, command_name)
        .iter()
        .for_each(|s| display(&command_example!(&s[..]), &display_format))
}
