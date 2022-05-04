use chrono::{Local, DateTime};
#[derive(Debug, Clone)]
pub struct Interview {
    date: DateTime<Local>,
}

pub struct Content {
    value: String,
}

pub struct FirstInterview {
    interview: Interview,
    content: Content,
}

pub struct SecondInterview {
    interview: Interview,
    content: Content,
}

pub struct ThirdInterview {
    interview: Interview,
    content: Content,
}

pub struct FinalInterview {
    interview: Interview,
    content: Content,
}