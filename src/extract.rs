pub enum PatternType<'a> {
    Phone,
    Email,
    Regex(&'a str),
}

impl PatternType<'_> {
    pub fn get_regexp(pattern: PatternType) -> String {
        match pattern {
            PatternType::Phone => String::from("\\(\\d{3}\\)\\s?\\d{3}[-.]\\d{4}|\\d{3}[-./\\s]\\d{3}[-.\\s]\\d{4}"),
            PatternType::Email => String::from("([0-9a-zA-Z]([-.\\w]*[0-9a-zA-Z])*@([0-9a-zA-Z][-\\w]*[0-9a-zA-Z]\\.)+[a-zA-Z]{2,9})"),
            PatternType::Regex(regexp) => String::from(regexp)
        }
    }
}

#[cfg(test)]
mod tests {
    use regex::{Match, Regex};

    use super::PatternType;

    #[test]
    fn test_extract_phone() {
        let html = "(000) 000-0000
            (000)000-0000
            000-000-0000
            000 000 0000
            000.000.0000
            000/000.0000

            0000000000
            (00) 000-0000
            000) 000-0000";

        if let Ok(regexp) = Regex::new(PatternType::get_regexp(PatternType::Phone).as_str()) {
            let actual: Vec<String> = regexp.find_iter(html)
                .map(|m: Match| String::from(m.as_str()))
                .collect();

            let expected: Vec<String> = vec![
                String::from("(000) 000-0000"),
                String::from("(000)000-0000"),
                String::from("000-000-0000"),
                String::from("000 000 0000"),
                String::from("000.000.0000"),
                String::from("000/000.0000"),
            ];

            assert_eq!(actual, expected);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_extract_email() {
        let html = "first.last@domain.com
            firstlast@domain.com

            @domain.com
            first.last@.com
            first.lastdomain.com
            first.last@domain";

        if let Ok(regexp) = Regex::new(PatternType::get_regexp(PatternType::Email).as_str()) {
            let actual: Vec<String> = regexp.find_iter(html)
                .map(|m: Match| String::from(m.as_str()))
                .collect();

            let expected: Vec<String> = vec![
                String::from("first.last@domain.com"),
                String::from("firstlast@domain.com"),
            ];

            assert_eq!(actual, expected);
        } else {
            panic!()
        }
    }
}