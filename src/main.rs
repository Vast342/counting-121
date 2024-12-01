use std::collections::{HashSet, VecDeque};

#[derive(Clone, Debug)]
enum Operation {
    Subtract1,
    Add5,
    MultiplyBy11,
    DivideBy7,
}
impl Operation {
    fn apply(&self, number: i64) -> Option<i64> {
        match self {
            Operation::Subtract1 => Some(number - 1),
            Operation::Add5 => Some(number + 5),
            Operation::MultiplyBy11 => Some(number * 11),
            Operation::DivideBy7 => {
                if number % 7 == 0 {
                    Some(number / 7)
                } else {
                    None
                }
            }
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Operation::Subtract1 => "-1",
            Operation::Add5 => "+5",
            Operation::MultiplyBy11 => "*11",
            Operation::DivideBy7 => "/7",
        }
    }
}

// breadth-first search go brrrr
fn find_path(start: i64, target: i64) -> Option<(Vec<Operation>, usize)> {
    let operations = vec![
        Operation::Subtract1,
        Operation::Add5,
        Operation::MultiplyBy11,
        Operation::DivideBy7,
    ];
    let mut visited = HashSet::new();
    visited.insert(start);
    // this is kinda interesting, never used it before
    let mut queue = VecDeque::new();
    queue.push_back((start, Vec::new()));
    const MAX_ITERATIONS: usize = 10_000;
    const MAX_NUMBER: i64 = 1_000_000;
    for _ in 0..MAX_ITERATIONS {
        let (current, path) = match queue.pop_front() {
            Some(state) => state,
            None => break,
        };
        if current == target {
            return Some((path.clone(), path.len()));
        }
        for op in &operations {
            let next_number = match op.apply(current) {
                Some(num) => num,
                None => continue,
            };
            if next_number.abs() > MAX_NUMBER {
                continue;
            }
            if visited.contains(&next_number) {
                continue;
            }
            visited.insert(next_number);
            let mut new_path = path.clone();
            new_path.push(op.clone());
            queue.push_back((next_number, new_path));
        }
    }
    None
}
fn find_and_print(start: i64, target: i64) {
    match find_path(start, target) {
        Some((path, steps)) => {
            println!("Transforming {} to {} in {} steps:", start, target, steps);
            let mut current = start;
            for op in path {
                current = op.apply(current).unwrap();
                println!("{}: {}", op.name(), current);
            }
            println!("{}", target);
        }
        None => println!("No path found from {} to {}", start, target),
    }
}

fn main() {
    // randomly selected first value (as in i googled "rng 1-100")
    find_and_print(30, 121);
}
