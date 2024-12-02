use itertools::Itertools;

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
            diffs.iter().all(|diff| diff.abs() > 0 && diff.abs() < 4)
            &&
            diffs.iter().map(|diff| diff.cmp(&0)).all_equal()
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
        assert_eq!(2, process(input)?);
        Ok(())
    }
}
