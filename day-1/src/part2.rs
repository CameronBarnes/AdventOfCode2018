use std::collections::HashSet;

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut set = HashSet::new();
    set.insert(0);
    let mut counter = 0;
    for offset in input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .cycle()
    {
        counter += offset;
        if !set.insert(counter) {
            break;
        }
    }
    counter.to_string()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("+1\n-1", "0")]
    #[case("+3\n+3\n+4\n-2\n-4", "10")]
    #[case("-6\n+3\n+8\n+5\n-6", "5")]
    #[case("+7\n+7\n-2\n-7\n-4", "14")]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process(input));
    }
}
