const fn swap_case(c: char) -> char {
    if c.is_ascii_uppercase() {
        c.to_ascii_lowercase()
    } else {
        c.to_ascii_uppercase()
    }
}

fn react(input: &str) -> usize {
    let mut out = Vec::with_capacity(input.len());
    input.trim_end().chars().for_each(|c| {
        if !out.is_empty() && *out.last().unwrap() == swap_case(c) {
            out.pop().unwrap();
        } else {
            out.push(c);
        }
    });
    out.len()
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    ('a'..='z')
        .map(|c| input.replace([c, c.to_ascii_uppercase()], ""))
        .map(|line| react(&line))
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "dabAcCaCBAcCcaDA";
        assert_eq!("4", process(input));
    }
}
