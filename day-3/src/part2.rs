use itertools::Itertools;

#[derive(Debug)]
struct Rect {
    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
}

impl Rect {
    const fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            left: x,
            right: x + width,
            top: y,
            bottom: y + height,
        }
    }

    fn overlaps(&self, other: &Self) -> bool {
        let left = self.left.max(other.left);
        let right = self.right.min(other.right);
        let top = self.top.max(other.top);
        let bottom = self.bottom.min(other.bottom);
        left < right && top < bottom
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let rects = input
        .lines()
        .map(|line| line.split_once(" @ ").unwrap())
        .map(|(id, line)| (id.strip_prefix('#').unwrap(), line.split_once(": ").unwrap()))
        .map(|(id, (coords, size))| {
            let (x, y) = coords.split_once(',').unwrap();
            let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
            let (width, height) = size.split_once('x').unwrap();
            let (width, height) = (width.parse().unwrap(), height.parse().unwrap());
            (id, Rect::new(x, y, width, height))
        })
        .collect_vec();

    for (id, a) in &rects {
        let mut collision = false;
        for (id_b, b) in &rects {
            if id.eq(id_b) {
                continue;
            }
            if a.overlaps(b) {
                collision = true;
                break;
            }
        }
        if !collision {
            return (*id).to_string();
        }
    }
    panic!("None found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";
        assert_eq!("3", process(input));
    }
}
