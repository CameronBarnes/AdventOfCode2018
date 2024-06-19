use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let result = input
        .lines()
        .map(|line| {
            let mut two = 0;
            let mut three = 0;
            for (_, chunk) in &line.chars().sorted().chunk_by(|c| *c) {
                match chunk.count() {
                    2 => two = 1,
                    3 => three = 1,
                    _ => {}
                };
            }
            (two, three)
        })
        .fold((0, 0), |acc, (two, three)| (acc.0 + two, acc.1 + three));
    (result.0 * result.1).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "abcdef
            bababc
            abbcde
            abcccd
            aabcdd
            abcdee
            ababab";
        assert_eq!("12", process(input));
    }
}
