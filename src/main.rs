use rand;
use rand::prelude::*;

use std::io::{stdin, Read};

mod types;

use types::*;

use std::collections::*;

use std::mem;

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let mut split = input.split(&[' ', '\n']);
    let mut word = || split.next().unwrap();

    let c = word().parse::<usize>().unwrap();
    let p = word().parse::<usize>().unwrap();

    let mut rng = rand::thread_rng();

    let contributors: Vec<types::Collaborator> = (0..c)
        .map(|_| {
            let name = word();
            let skills = (0usize..word().parse().unwrap())
                .map(|_| (word(), word().parse::<usize>().unwrap()))
                .collect();
            types::Collaborator {
                name,
                skills,
                occupied_until: 0,
            }
        })
        .collect();

    let mut projects: Vec<types::Project> = (0..p)
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

    let mut remaining = Vec::new();

    let mut people: Vec<_> = contributors;

    let mut plan: Vec<Assignment> = Vec::new();

    let mut end_times: BinaryHeap<OrderedAssignment> = BinaryHeap::new();

    let mut time = 0;

    projects.sort_by_key(|p| p.best_before - p.length);

    let mut i = 0;

    'outer: loop {
        if i % 5 == 0 {
            eprintln!("iter: {}, time: {}", i, time);
        }

        //projects.shuffle(&mut rng);
        projects.sort_by_key(|p| p.best_before - p.length);

        'proj: for project in projects.drain(..) {
            if rng.gen_range(0..100) < 5 {
                continue;
            }

            let mut chosen = Vec::new();

            for &(skill_name, required) in &project.skills {
                let mut closest_match: Option<(usize, usize)> = None;
                let required = if chosen
                    .iter()
                    .map(|c| &people[*c])
                    .any(|p: &Collaborator| p.skills.get(skill_name).unwrap_or(&0) >= &required)
                {
                    required - 1
                } else {
                    required
                };

                for (c, coll) in people.iter_mut().enumerate() {
                    let skill_level = coll.skills.get(skill_name).unwrap_or(&0);
                    if skill_level >= &required {
                        if let Some(closest) = closest_match {
                            if skill_level > &closest.1 && coll.occupied_until <= time {
                                closest_match = Some((c, *skill_level));
                            }
                        } else {
                            if !chosen.contains(&c) {
                                closest_match = Some((c, *skill_level));
                            }
                        }
                    }
                }

                if let Some(closest) = closest_match {
                    people[closest.0].occupied_until = time + project.length;
                    chosen.push(closest.0);
                } else {
                    for choice in chosen {
                        people[choice].occupied_until = time;
                    }
                    remaining.push(project);
                    continue 'proj;
                }
            }

            let end_time = time + project.length;

            let mut cur = Assignment {
                collaborators: chosen,
                project,
                end_time,
            };

            for (person, skill) in cur.collaborators.iter().zip(cur.project.skills.iter()) {
                let s = people[*person].skills.entry(skill.0).or_insert(0);
                if *s <= skill.1 {
                    *s += 1;
                }
            }

            end_times.push(OrderedAssignment(end_time));

            plan.push(cur.clone());
        }

        std::mem::swap(&mut remaining, &mut projects);

        let next_time = match end_times.pop() {
            Some(x) => x.0,
            None => break 'outer,
        };

        time = next_time;
        i += 1;

        while let Some(i) = end_times.peek() {
            if i.0 == next_time {
                end_times.pop();
            } else {
                break;
            }
        }
    }

    println!("{}", plan.len());
    let mut sum = 0;
    let n = plan.len();
    for assignment in plan {
        println!("{}", assignment.project.name);
        println!(
            "{}",
            assignment
                .collaborators
                .into_iter()
                .map(|i| people[i].name)
                .collect::<Vec<_>>()
                .join(" ")
        );
        sum += score(assignment.end_time, &assignment.project).max(0);
    }
    eprintln!("score: {}, projs: {}", sum, n);
}

fn score(time: usize, p: &Project) -> i64 {
    let &Project {
        score,
        length,
        best_before,
        ..
    } = p;

    (score as i64).min((score as i64 - ((best_before - length) - time) as i64))
}

fn urgency(time: usize, p: &Project) -> usize {
    let &Project {
        score,
        length,
        best_before,
        ..
    } = p;

    let e = score.min(score - ((best_before - length) - time));
    ((e as f64 / (best_before - length) as f64) * 10000.0) as usize
}
