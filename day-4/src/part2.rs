use std::collections::HashMap;

use itertools::Itertools;

enum Data {
    Start(usize),
    Sleep,
    Wake,
}

#[derive(Default, Debug)]
struct Shift {
    ranges: Vec<(usize, usize)>,
}

impl Shift {
    fn most_asleep(&self) -> (usize, usize) {
        if self.ranges.is_empty() {
            return (0, 0);
        }
        let mut earliest_sleep = usize::MAX;
        let mut latest_wake = 0;
        self.ranges.iter().for_each(|(start, end)| {
            earliest_sleep = earliest_sleep.min(*start);
            latest_wake = latest_wake.max(*end);
        });

        let size = (latest_wake - earliest_sleep) + 1;
        let mut vec = vec![0; size];
        self.ranges
            .iter()
            .flat_map(|(start, end)| *start..*end)
            .for_each(|minute| {
                vec[minute - earliest_sleep] += 1;
            });
        let mut best_minute = 0;
        let mut best_count = 0;
        for (minute, count) in vec.into_iter().enumerate() {
            if count > best_count {
                best_minute = minute + earliest_sleep;
                best_count = count;
            }
        }
        (best_minute, best_count)
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut shifts: HashMap<usize, Shift> = HashMap::new();
    let mut current_id = 0;
    let mut last_time = 0;
    for (timestamp, data) in input
        .lines()
        .map(|line| line.split_once(']').unwrap())
        .map(|(timestamp, data)| {
            let data = if let Some((_, id_part)) = data.split_once('#') {
                let id = id_part.split_once(' ').unwrap().0.parse().unwrap();
                Data::Start(id)
            } else if data.eq(" falls asleep") {
                Data::Sleep
            } else {
                Data::Wake
            };
            (timestamp, data)
        })
        .sorted_by_key(|(timestamp, _data)| *timestamp)
    {
        let current = shifts.entry(current_id).or_default();
        match data {
            Data::Start(id) => {
                current_id = id;
            }
            Data::Sleep => {
                last_time = timestamp.split_once(':').unwrap().1.parse().unwrap();
            }
            Data::Wake => {
                let current_time: usize = timestamp.split_once(':').unwrap().1.parse().unwrap();
                current.ranges.push((last_time, current_time));
            }
        }
    }

    let mut best_id = 0;
    let mut best_minute = 0;
    let mut max_count = 0;
    for (id, shift) in &shifts {
        let (minute, count) = shift.most_asleep();
        if count > max_count {
            best_id = *id;
            max_count = count;
            best_minute = minute;
        }
    }

    (best_id * best_minute).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
        assert_eq!("4455", process(input));
    }
}
