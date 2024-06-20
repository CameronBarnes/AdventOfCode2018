use std::collections::HashSet;

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

    fn intersection(&self, other: &Self) -> Option<Self> {
        let left = self.left.max(other.left);
        let right = self.right.min(other.right);
        let top = self.top.max(other.top);
        let bottom = self.bottom.min(other.bottom);
        if left < right && top < bottom {
            Some(Self {
                left,
                right,
                top,
                bottom,
            })
        } else {
            None
        }
    }

    const fn area(&self) -> usize {
        (self.right - self.left) * (self.bottom - self.top)
    }

    fn cells(&self) -> Vec<(usize, usize)> {
        let mut vec = Vec::with_capacity(self.area());
        for x in self.left..self.right {
            for y in self.top..self.bottom {
                vec.push((x, y));
            }
        }
        vec
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let rects = input
        .lines()
        .map(|line| line.split_once(" @ ").unwrap().1)
        .map(|line| line.split_once(": ").unwrap())
        .map(|(coords, size)| {
            let (x, y) = coords.split_once(',').unwrap();
            let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
            let (width, height) = size.split_once('x').unwrap();
            let (width, height) = (width.parse().unwrap(), height.parse().unwrap());
            Rect::new(x, y, width, height)
        })
        .collect_vec();

    let mut cell_set = HashSet::new();
    for (i, a) in rects.iter().enumerate() {
        for b in rects.iter().skip(i + 1) {
            if let Some(rect) = a.intersection(b) {
                rect.cells().into_iter().for_each(|cell| {
                    cell_set.insert(cell);
                });
            }
        }
    }
    //dbg!(&cell_set);
    cell_set.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";
        assert_eq!("4", process(input));
    }
}
