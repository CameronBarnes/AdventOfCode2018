use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn parse_line(line: &str) -> (usize, usize) {
    let tmp = line.split_once(", ").unwrap();
    (tmp.0.parse().unwrap(), tmp.1.parse().unwrap())
}

#[derive(Clone, Debug)]
enum Cell {
    None,
    One { distance: usize, num: usize },
    Tie(usize),
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let coords = input.lines().map(parse_line).collect_vec();
    let mut min = (0, 0);
    let mut max = (0, 0);
    for (x, y) in &coords {
        min = (min.0.min(*x), min.1.min(*y));
        max = (max.0.max(*x), max.1.max(*y));
    }

    let mut cells = vec![vec![Cell::None; max.1 - min.1 + 1]; max.0 - min.0 + 1];

    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    let distance = |(x, y): (usize, usize), (xx, yy): (usize, usize)| {
        ((x as isize - xx as isize).abs() + (y as isize - yy as isize).abs()) as usize
    };

    let vec_max_0 = cells.len();
    let vec_max_1 = cells[0].len();

    for (counter, (x, y)) in coords.iter().enumerate() {
        let x = x - min.0;
        let y = y - min.1;
        (0..vec_max_0).for_each(|xx| {
            for yy in 0..vec_max_1 {
                let cell = &mut cells[xx][yy];
                let new_dist = distance((x, y), (xx, yy));
                match cell {
                    Cell::None => {
                        *cell = Cell::One {
                            distance: distance((x, y), (xx, yy)),
                            num: counter,
                        };
                    }
                    Cell::One {
                        distance: dist,
                        num: _,
                    } if *dist == new_dist => {
                        *cell = Cell::Tie(new_dist);
                    }
                    Cell::One {
                        distance: dist,
                        num: _,
                    } if *dist > new_dist => {
                        *cell = Cell::One {
                            distance: new_dist,
                            num: counter,
                        };
                    }
                    Cell::One {
                        distance: _,
                        num: _,
                    } => {} // Existing cell is already closer to another point
                    Cell::Tie(dist) => {
                        if *dist > new_dist {
                            *cell = Cell::One {
                                distance: new_dist,
                                num: counter,
                            };
                        }
                    }
                }
            }
        });
    }

    let mut counts = HashMap::new();
    let mut infinite = HashSet::new();

    (0..vec_max_0).for_each(|x| {
        for y in 0..vec_max_1 {
            #[allow(clippy::match_on_vec_items)]
            match cells[x][y] {
                Cell::None => unreachable!(), // All cells were updated in previous loop
                Cell::One { distance: _, num } => {
                    if x == 0 || x == vec_max_0 - 1 || y == 0 || y == vec_max_1 - 1 {
                        infinite.insert(num);
                        counts.remove(&num);
                    } else if !infinite.contains(&num) {
                        counts.entry(num).and_modify(|count| *count += 1).or_insert(1_usize);
                    }
                }
                Cell::Tie(_) => {} // We're doing nothing here because tie doesnt store the
                                   // index of the source coords, we'll adjust this if it
                                   // causes issues
            }
        }
    });

    counts.values().max().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
        assert_eq!("17", process(input));
    }
}
