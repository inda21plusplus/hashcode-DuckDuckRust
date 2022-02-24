use std::collections::HashMap;

use std::cmp::*;

#[derive(Clone, Debug)]
pub struct Project<'i> {
    pub name: &'i str,
    pub skills: Vec<(&'i str, usize)>,
    pub length: usize,
    pub score: usize,
    pub best_before: usize,
}

#[derive(Clone, Debug)]
pub struct Collaborator<'i> {
    pub name: &'i str,
    pub skills: HashMap<&'i str, usize>,
    pub occupied_until: usize,
}

#[derive(Clone, Debug)]
pub struct Assignment<'a> {
    pub collaborators: Vec<usize>,
    pub project: Project<'a>,
    pub end_time: usize,
}

#[derive(Clone, Debug)]
pub struct OrderedAssignment(pub usize);

impl<'a> PartialEq for OrderedAssignment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<'a> Eq for OrderedAssignment {}

impl<'a> PartialOrd for OrderedAssignment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0).map(Ordering::reverse)
    }
}

impl<'a> Ord for OrderedAssignment {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

#[derive(Clone, Debug)]
pub struct ProjectPlan<'a> {
    pub assignments: Vec<Assignment<'a>>,
}
