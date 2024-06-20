use itertools::Itertools;

fn compare(first: &str, second: &str) -> Option<String> {
    let vec = first
        .chars()
        .zip(second.chars()).enumerate()
        .filter_map(|(index, (a, b))| a.ne(&b).then_some(index))
        .collect_vec();
    if vec.len() == 1 {
        let mut first = first.to_string();
        first.remove(vec[0]);
        Some(first)
    } else {
        None
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let lines = input.lines().collect_vec();

    for (i, a) in lines.iter().enumerate() {
        for b in lines.iter().skip(i + 1) {
            if let Some(out) = compare(a, b) {
                return out;
            }
        }
    }
    panic!("None found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";
        assert_eq!("fgij", process(input));
    }
}
