use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (mut a, mut b): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Input should be ints"))
            .collect_tuple::<(i32, i32)>()
            .expect("Expected two items split by whitespace")
        )
        .unzip();
    a.sort();
    b.sort();
    let sum: i32 = a.into_iter()
        .zip(b)
        .map(|(a, b)| (a - b).abs())
        .sum();
    Ok(format!("{sum}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
