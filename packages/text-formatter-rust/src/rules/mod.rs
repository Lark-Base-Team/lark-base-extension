use crate::utils::{byte_range_to_char_range, vec_to_option};

pub trait Rule {
    fn id(&self) -> &str;
    fn test(&self, text: &str) -> Option<Vec<(usize, usize)>>;
    fn fix(&self, text: &str) -> String;
}

pub struct RegexRule {
    id: String,
    test: Vec<regex::Regex>,
    fixer: Vec<(regex::Regex, String)>,
}

impl RegexRule {
    fn new(id: &str, test: Vec<&str>, fixer: Vec<(&str, &str)>) -> Self {
        Self {
            id: id.into(),
            test: test
                .into_iter()
                .map(|regex| regex::Regex::new(regex).unwrap())
                .collect(),
            fixer: fixer
                .into_iter()
                .map(|(regex, replacement)| (regex::Regex::new(regex).unwrap(), replacement.into()))
                .collect(),
        }
    }
}

impl Rule for RegexRule {
    fn id(&self) -> &str {
        &self.id
    }

    fn test(&self, text: &str) -> Option<Vec<(usize, usize)>> {
        vec_to_option(
            self.test
                .iter()
                .flat_map(|regex| {
                    regex.captures_iter(text).flat_map(|caps| {
                        caps.name("h")
                            .iter()
                            .map(|m| byte_range_to_char_range(text, (m.start(), m.end())))
                            .collect::<Vec<_>>()
                    })
                })
                .collect::<Vec<_>>(),
        )
        .map(|mut v| {
            v.sort();
            v
        })
    }

    fn fix(&self, text: &str) -> String {
        let mut result = text.to_string();
        for (regex, replacement) in &self.fixer {
            result = regex.replace_all(&result, replacement).to_string();
        }
        result
    }
}

pub mod no_space_around_full_width_punctuation;
pub mod no_space_between_num_dp;
pub mod space_between_ch_en;
pub mod uniform_punctuation;
