use crate::commands::command::CommandExample;
use crate::commands::database::CommandsDB;
use crate::commands::new_command::publish_new_command;
use crate::examples::*;
use crate::query::fuzzy_search::{fuzzy_search, FuzzySearchCategory};
use crate::utils::display::{display, DisplayFormat};

use clap::{App, Arg};

pub fn run() {
    let db = CommandsDB::new();

    let matches = App::new("Command Example")
        .version("0.1.12")
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
            Arg::with_name("publish-example")
                .short("p")
                .long("publish-example")
                .value_name("PUBLISH EXAMPLE")
                .help("Add a .json file with the example you want to publish")
                .case_insensitive(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("query-category")
                .short("c")
                .long("category")
                .value_name("QUERY CATEGORY")
                .help("Query a command on a category (fuzzy match)")
                .possible_values(&FuzzySearchCategory::variants())
                .case_insensitive(true)
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
                .takes_value(true),
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
    let query_category = matches
        .value_of("query-category")
        .map(|s| match s {
            "command" => FuzzySearchCategory::Command,
            _ => FuzzySearchCategory::Description,
        })
        .unwrap_or(FuzzySearchCategory::Description);
    let display_format = match matches.value_of("display_format").unwrap_or("ascii") {
        "ascii" => DisplayFormat::ASCII,
        "json" => DisplayFormat::JSON,
        _ => DisplayFormat::ASCII,
    };
    if let Some(new_example_path_str) = matches.value_of("publish-example") {
        match publish_new_command(new_example_path_str) {
            Ok(_pr_url) => {
                //println!("Congratulations! You just posted a new command example. Check it on {}", pr_url);
                println!("Congratulations! Your command is ready to be published. Submit a PR on our github repo: https://github.com/blasrodri/command_example.git");
                return;
            }
            Err(e) => {
                println!("{}", e);
                return;
            }
        }
    }
    match (command_name, query_fuzzy) {
        (Some(cmd_opt), Some(fuzzy_query)) => find_examples_fuzzy(
            &db,
            fuzzy_query,
            Some(cmd_opt),
            &display_format,
            query_category,
        ),
        (None, Some(fuzzy_query)) => find_examples_fuzzy(
            &db,
            fuzzy_query,
            None,
            &display_format,
            FuzzySearchCategory::Description,
        ),
        (Some(cmd), _) => find_examples(&db, cmd, &display_format),
        _ => eprintln!("Unrecognized command. Run command_examples --help for more information."),
    }
}

fn find_examples(db: &CommandsDB, command_name: &str, display_format: &DisplayFormat) {
    match db.get_data().get(command_name) {
        Some(values) => examples::examples(values, &display_format),
        None => println!("{}", format!("No command examples for {}.", command_name)),
    }
}

fn find_examples_fuzzy(
    db: &CommandsDB,
    query: &str,
    command_name: Option<&str>,
    display_format: &DisplayFormat,
    category: FuzzySearchCategory,
) {
    let results = fuzzy_search(db, query, command_name, category);
    if results.clone().iter().count() == 0 {
        println!("{}", format!("No command examples for query {}.", query));
    } else {
        results
            .iter()
            .for_each(|s| display(&command_example!(s.as_str()), &display_format))
    }
}
