use std::fmt::Display;

#[derive(Debug)]
pub struct Comic {
    pub title: String,
    pub number: u32,
    pub date: String,
    pub description: String,
    pub image: String,
}

impl Display for Comic {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "title: {}, number: {}", self.title, self.number)
    }
}
