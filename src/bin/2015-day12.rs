#![allow(clippy::trivial_regex)]

use regex::bytes::Regex;

use std::fs;

fn count(re: &Regex, input: &[u8]) -> i32 {
    re.find_iter(&input)
        .map(|x| -> i32 { String::from_utf8_lossy(x.as_bytes()).parse().unwrap() })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = fs::read("inputs/2015-day12.txt")?;

    let regex_num = Regex::new(r#"-?\d+"#).unwrap();
    let regex_red = Regex::new(r#":"red""#).unwrap();

    let result1 = count(&regex_num, &input);

    while let Some(x) = regex_red.find_iter(&input).next() {
        let before = input[..x.start()]
            .iter()
            .rev()
            .enumerate()
            .scan(-1, |braces, (pos, c)| {
                match c {
                    b'{' => *braces += 1,
                    b'}' => *braces -= 1,
                    _ => {}
                };
                Some((pos, *braces))
            })
            .find(|&(_, braces)| braces == 0)
            .map(|(pos, _)| pos)
            .unwrap();

        let after = input[x.end()..]
            .iter()
            .enumerate()
            .scan(1, |braces, (pos, c)| {
                match c {
                    b'{' => *braces += 1,
                    b'}' => *braces -= 1,
                    _ => {}
                };
                Some((pos, *braces))
            })
            .find(|&(_, braces)| braces == 0)
            .map(|(pos, _)| pos)
            .unwrap();

        let range = (x.start() - before)..(x.end() + after);
        input[range].fill(b' ');
    }

    let result2 = count(&regex_num, &input);

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
