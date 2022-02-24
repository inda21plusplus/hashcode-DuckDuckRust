use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Project<'i> {
    pub name: &'i str,
    pub skills: Vec<(&'i str, u64)>,
    pub length: usize,
    pub score: usize,
    pub best_before: usize,
}

#[derive(Clone, Debug)]
pub struct Collaborator<'i> {
    pub name: &'i str,
    pub skills: HashMap<&'i str, u64>,
}

#[derive(Clone, Debug)]
pub struct Assignment {
    pub collaborators: Vec<Collaborator>,
    pub project: Project,
}

#[derive(Clone, Debug)]
pub struct ProjectPlan {
    pub assignments: Vec<Assignment>,
}
