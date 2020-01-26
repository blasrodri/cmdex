use crate::commands::command::*;
use crate::utils::load_files::{load_command_examples_content, load_json_file_paths};

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub fn fuzzy_search<'a>(query: &'a str, command_name: Option<&'a str>) -> Vec<String> {
    let list_json_paths = load_json_file_paths(command_name);
    let list_command_examples_content = load_command_examples_content(&list_json_paths);
    let matcher = SkimMatcherV2::default();
    let mut list_command_examples_with_match_points = list_command_examples_content
        .iter()
        .map(|command_ex_str| {
            (
                matcher.fuzzy_match(command_example!(command_ex_str).description, query),
                command_ex_str,
            )
        })
        .filter(|(opt_points, _)| opt_points.is_some())
        .map(|(opt_points, s)| (opt_points.unwrap(), s))
        .collect::<Vec<(_, _)>>();
    list_command_examples_with_match_points.sort_by(|a, b| a.0.cmp(&b.0));
    list_command_examples_with_match_points
        .iter()
        .map(|a| a.1.to_string())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fuzzy_search_wo_command_name() {
        let expected =
            vec![
                include_str!("../examples/examples-data/find/find-contains-filename-in-cwd.json")
                    .to_string(),
            ];
        let query = "name is foo.";
        assert_eq!(fuzzy_search(query, None), expected);
    }

    #[test]
    fn test_fuzzy_search_w_command_name() {
        let expected =
            vec![
                include_str!("../examples/examples-data/find/find-contains-filename-in-cwd.json")
                    .to_string(),
            ];
        let query = "find all the files name is foo.";
        assert_eq!(fuzzy_search(query, Some("find")), expected);
        assert_eq!(fuzzy_search(query, Some("grep")), Vec::<String>::new())
    }
}
