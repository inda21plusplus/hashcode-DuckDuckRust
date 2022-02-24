use std::collections::HashMap;

pub struct Project {
    pub skills: HashMap<String, u64>,
    // in days
    pub length: usize,
    pub best_before: usize,
    pub n_roles: usize,
}

pub struct Collaborator {
    pub skills: HashMap<String, u64>,
}
