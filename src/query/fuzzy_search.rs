use crate::commands::command::*;
use crate::commands::database::CommandsDB;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

arg_enum! {
    #[derive(Debug)]
    pub enum FuzzySearchCategory {
        Description,
        Command,
    }
}

pub fn fuzzy_search<'a>(
    db: &CommandsDB,
    query: &'a str,
    command_name: Option<&'a str>,
    category: FuzzySearchCategory,
) -> Vec<String> {
    let list_command_examples_content = match command_name {
        None => db
            .get_data()
            .values()
            .flatten()
            .collect::<Vec<&CommandExample>>()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        Some(cmd) => db
            .get_data()
            .get(cmd)
            .unwrap()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
    };
    let matcher = SkimMatcherV2::default();
    let mut list_command_examples_with_match_points = list_command_examples_content
        .iter()
        .map(|command_ex_str| {
            (
                matcher.fuzzy_match(
                    &command_example!(command_ex_str).get_by_category(&category),
                    query,
                ),
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

    use crate::commands::database::CommandsDB;

    #[test]
    fn test_fuzzy_search_wo_command_name() {
        let expected = vec![command_example!(include_str!(
            "../../examples-data/find/find-contains-filename-in-cwd.json"
        ))
        .to_string()];
        let query = "name is foo.";
        assert_eq!(
            fuzzy_search(
                &CommandsDB::new(),
                query,
                None,
                FuzzySearchCategory::Description
            ),
            expected
        );
    }

    #[test]
    fn test_fuzzy_search_w_command_name() {
        let expected = vec![command_example!(include_str!(
            "../../examples-data/find/find-contains-filename-in-cwd.json"
        ))
        .to_string()];
        let query = "find all the files name is foo.";
        assert_eq!(
            fuzzy_search(
                &CommandsDB::new(),
                query,
                Some("find"),
                FuzzySearchCategory::Description
            ),
            expected
        );
        assert_eq!(
            fuzzy_search(
                &CommandsDB::new(),
                query,
                Some("grep"),
                FuzzySearchCategory::Description
            ),
            Vec::<String>::new()
        )
    }
}
