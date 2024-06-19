#[tracing::instrument]
pub fn process(input: &str) -> String {
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!("3", process("+1\n+1\n+1"));
        assert_eq!("8", process("+2\n+8\n-2"));
        assert_eq!("-6", process("-1\n-2\n-3"));
    }
}
