use miette::IntoDiagnostic;
use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").into_diagnostic()?;
    let mut total = 0u32;
    let mut enabled = true;
    for captures in re.captures_iter(input) {
        if let Some(m) = captures.get(0) {
            match m.as_str() {
                "do()" => {
                    enabled = true;
                }
                "don't()" => {
                    enabled = false;
                }
                _ => {
                    if enabled {
                        let a = captures.get(1)
                            .expect("mul() regex has two submatches")
                            .as_str()
                            .parse::<u32>()
                            .into_diagnostic()?;
                        let b = captures.get(2)
                            .expect("mul() regex has two submatches")
                            .as_str()
                            .parse::<u32>()
                            .into_diagnostic()?;
                        total += a * b;
                    }
                }
            }
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(48, process(input)?);
        Ok(())
    }
}
