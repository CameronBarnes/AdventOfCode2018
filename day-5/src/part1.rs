const fn swap_case(c: char) -> char {
    if c.is_ascii_uppercase() {
        c.to_ascii_lowercase()
    } else {
        c.to_ascii_uppercase()
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut out = Vec::with_capacity(input.len());
    input.trim_end().chars().for_each(|c| {
        if !out.is_empty() && *out.last().unwrap() == swap_case(c) {
            out.pop().unwrap();
        } else {
            out.push(c);
        }
    });
    out.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "dabAcCaCBAcCcaDA";
        assert_eq!("10", process(input));
    }
}
