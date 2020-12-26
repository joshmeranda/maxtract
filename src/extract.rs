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
