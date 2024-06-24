use itertools::Itertools;

fn parse_line(line: &str) -> (usize, usize) {
    let tmp = line.split_once(", ").unwrap();
    (tmp.0.parse().unwrap(), tmp.1.parse().unwrap())
}

#[tracing::instrument]
pub fn process(input: &str, range: usize) -> String {
    let coords = input.lines().map(parse_line).collect_vec();
    let mut min = (0, 0);
    let mut max = (0, 0);
    for (x, y) in &coords {
        min = (min.0.min(*x), min.1.min(*y));
        max = (max.0.max(*x), max.1.max(*y));
    }

    let mut cells = vec![vec![0usize; max.1 - min.1 + 1]; max.0 - min.0 + 1];

    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    let distance = |(x, y): (usize, usize), (xx, yy): (usize, usize)| {
        ((x as isize - xx as isize).abs() + (y as isize - yy as isize).abs()) as usize
    };

    let vec_max_0 = cells.len();
    let vec_max_1 = cells[0].len();

    for (x, y) in &coords {
        let x = x - min.0;
        let y = y - min.1;
        (0..vec_max_0).for_each(|xx| {
            for yy in 0..vec_max_1 {
                cells[xx][yy] += distance((x, y), (xx, yy));
            }
        });
    }

    cells
        .iter()
        .flatten()
        .filter(|count| **count < range)
        .count()
        .to_string()
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
        assert_eq!("16", process(input, 32));
    }
}
