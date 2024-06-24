use std::{cmp::Reverse, collections::HashMap};

use itertools::Itertools;

fn parse_line(line: &str) -> (char, char) {
    line.strip_prefix("Step ")
        .unwrap()
        .replace(" must be finished before step ", "")
        .strip_suffix(" can begin.")
        .unwrap()
        .trim()
        .chars()
        .next_tuple()
        .unwrap()
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut map = HashMap::new();
    input.lines().map(parse_line).for_each(|(req, name)| {
        map.entry(req).or_default();
        map.entry(name)
            .and_modify(|vec: &mut Vec<char>| vec.push(req))
            .or_insert_with(|| vec![req]);
    });

    let mut out = Vec::new();
    let mut next = map
        .iter()
        .filter_map(|(name, req)| if req.is_empty() { Some(*name) } else { None })
        .collect_vec();
    map.retain(|_name, req| !req.is_empty());
    while let Some(n) = next.pop() {
        out.push(n);
        for (name, req) in &mut map {
            req.retain(|val| *val != n);
            if req.is_empty() {
                next.push(*name);
            }
        }
        map.retain(|_name, req| !req.is_empty());
        next.sort_by_key(|val| Reverse(*val));
    }

    out.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
        assert_eq!("CABDFE", process(input));
    }
}
