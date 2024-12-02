use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

fn check_all(diffs: &[i32]) -> bool {
    diffs.iter().all(|diff| diff.abs() > 0 && diff.abs() < 4)
    &&
    diffs.iter().map(|diff| diff.cmp(&0)).all_equal()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize> {
    let count = input
        .lines()
        .map(|report| report
            .split_whitespace()
            .map(|level| level.parse::<i32>().expect("Input should be integers"))
            .collect::<Vec<_>>()
        )
        .filter(|levels| {
            let diffs = levels
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect::<Vec<_>>();
            let mut ordering_map = HashMap::from([
                (Ordering::Less, false),
                (Ordering::Equal, false),
                (Ordering::Greater, false),
            ]);
            let primary_ordering = diffs.iter()
                .find_map(|diff| {
                    let ord = diff.cmp(&0);
                    if *ordering_map.get(&ord).unwrap() {
                        Some(ord)
                    } else {
                        *ordering_map.get_mut(&ord).unwrap() = true;
                        None
                    }
                })
                .unwrap();
            let mut bad_diff_index = None;
            for (index, diff) in diffs.iter().enumerate() {
                if diff.cmp(&0) != primary_ordering || diff.abs() < 1 || diff.abs() > 3 {
                    bad_diff_index = Some(index);
                    break;
                }
            }
            if let Some(bad_diff_index) = bad_diff_index {
                // TODO: check for possible optimizations since we are brute forcing the two posiible indexes
                let mut without_first = levels.clone();
                without_first.remove(bad_diff_index);
                let diffs = without_first
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect::<Vec<_>>();
                if check_all(&diffs) {
                    true
                } else {
                    let mut without_second = levels.clone();
                    without_second.remove(bad_diff_index + 1);
                    let diffs = without_second
                        .iter()
                        .tuple_windows()
                        .map(|(a, b)| b - a)
                        .collect::<Vec<_>>();
                    check_all(&diffs)
                }
            } else {
                check_all(&diffs)
            }
        })
        .collect::<Vec<_>>()
        .len();
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(4, process(input)?);
        Ok(())
    }
}
