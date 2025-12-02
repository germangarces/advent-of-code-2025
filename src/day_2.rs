use crate::reader;
use anyhow::{Error, Result, bail};
use fancy_regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
struct Range {
    min: u64,
    max: u64,
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (min, max) = s
            .split_once('-')
            .ok_or_else(|| anyhow::anyhow!("invalid range"))?;
        Ok(Self {
            min: min.parse()?,
            max: max.trim_end().parse()?,
        })
    }
}

fn is_hit(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let re = Regex::new(r"^(.+)\1+$").unwrap();
    re.is_match(s).unwrap_or(false)
}

pub fn solution() -> u64 {
    reader::BufReader::open("./src/inputs/day_2")
        .expect("Error reading file")
        .fold(0u64, |acc, line| {
            let range: Range = line.unwrap().parse().unwrap();
            let mut local_acc = 0;

            for n in range.min..=range.max {
                if is_hit(n.to_string().as_str()) {
                    local_acc += n;
                }
            }

            acc + local_acc
        })
}
