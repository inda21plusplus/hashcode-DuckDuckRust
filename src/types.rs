use std::collections::HashMap;

pub struct Project<'i> {
    pub name: &'i str,
    pub skills: HashMap<&'i str, u64>,
    // in days
    pub length: usize,
    pub score: usize,
    pub best_before: usize,
}

pub struct Collaborator<'i> {
    pub skills: HashMap<&'i str, u64>,
}
