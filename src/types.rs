use std::collections::HashMap;

pub struct Project<'a> {
    pub name: &'a str,
    pub skills: HashMap<String, u64>,
    // in days
    pub length: usize,
    pub score: usize,
    pub best_before: usize,
}

pub struct Collaborator {
    pub skills: HashMap<String, u64>,
}
