use std::io::{stdin, Read};

mod types;

use types::*;

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let mut split = input.split(&[' ', '\n']);
    let mut word = || split.next().unwrap();

    let c = word().parse::<usize>().unwrap();
    let p = word().parse::<usize>().unwrap();

    let contributors: Vec<types::Collaborator> = (0..c)
        .map(|_| {
            let name = word();
            let skills = (0usize..word().parse().unwrap())
                .map(|_| (word(), word().parse::<u64>().unwrap()))
                .collect();
            types::Collaborator { name, skills }
        })
        .collect();

    let projects: Vec<types::Project> = (0..p)
        .map(|_| {
            let name = word();
            let length = word().parse().unwrap();
            let score = word().parse().unwrap();
            let best_before = word().parse().unwrap();
            let r_count = word().parse().unwrap();

            let skills = (0usize..r_count)
                .map(|_| {
                    let skillname = word();
                    let level = word().parse().unwrap();
                    (skillname, level)
                })
                .collect();
            types::Project {
                name,
                length,
                score,
                best_before,
                skills,
            }
        })
        .collect();
}
