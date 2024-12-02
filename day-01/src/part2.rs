use std::collections::HashMap;

use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (a, b): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Input should be ints"))
            .collect_tuple::<(i32, i32)>()
            .expect("Expected two items split by whitespace")
        )
        .unzip();
    // value, (value * occurance in a, occurance in b)
    let mut map = HashMap::<i32, (i32, i32)>::new();
    for val in a {
        map.entry(val)
            .and_modify(|e| e.0 += val)
            .or_insert((val, 0));
    }
    for val in b {
        if let Some(current) = map.get_mut(&val) {
            current.1 += 1;
        }
    }
    let sum: i32 = map
        .values()
        .map(|(val, count)| val * count)
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
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
