use crate::reader;
use std::collections::HashMap;

fn func(val: &str, digits: usize, memo: &mut HashMap<(String, usize), u64>) -> u64 {
    // Memoization lookup
    let key = (val.to_string(), digits);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    // Base case: nothing to take
    if digits == 0 {
        memo.insert(key, 0);
        return 0;
    }

    // If we must take all remaining digits
    if val.len() == digits {
        let v = val.parse::<u64>().unwrap();
        memo.insert(key, v);
        return v;
    }

    // Take the first digit
    let first_digit = val.chars().next().unwrap().to_digit(10).unwrap() as u64;
    let a = first_digit * 10u64.pow((digits - 1) as u32) + func(&val[1..], digits - 1, memo);

    // Skip the first digit
    let b = func(&val[1..], digits, memo);

    let result = a.max(b);
    memo.insert(key, result);
    result
}

pub fn solution() -> u64 {
    let mut memo = HashMap::new();

    reader::BufReader::open("./src/inputs/day_3")
        .expect("Error reading file")
        .fold(0u64, |acc, line| {
            let line = line.unwrap();
            let line = line.trim_end();
            acc + func(line, 12, &mut memo)
        })
}
