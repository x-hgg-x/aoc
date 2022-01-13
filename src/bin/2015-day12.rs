use aoc::*;

use regex::bytes::Regex;

fn count(re: &Regex, input: &[u8]) -> Result<i64> {
    re.find_iter(input).map(|x| Ok(String::from_utf8_lossy(x.as_bytes()).parse::<i64>()?)).try_process(|iter| iter.sum())
}

fn main() -> Result<()> {
    let mut input = setup(file!())?;

    let regex_num = Regex::new(r#"-?\d+"#)?;
    let regex_red = Regex::new(r#":"red""#)?;

    let result1 = count(&regex_num, &input)?;

    while let Some(x) = regex_red.find_iter(&input).next() {
        let before = input[..x.start()]
            .iter()
            .rev()
            .enumerate()
            .scan(-1, |braces, (pos, c)| {
                match c {
                    b'{' => *braces += 1,
                    b'}' => *braces -= 1,
                    _ => (),
                };
                Some((pos, *braces))
            })
            .find(|&(_, braces)| braces == 0)
            .map(|(pos, _)| pos)
            .value()?;

        let after = input[x.end()..]
            .iter()
            .enumerate()
            .scan(1, |braces, (pos, c)| {
                match c {
                    b'{' => *braces += 1,
                    b'}' => *braces -= 1,
                    _ => (),
                };
                Some((pos, *braces))
            })
            .find(|&(_, braces)| braces == 0)
            .map(|(pos, _)| pos)
            .value()?;

        let range = (x.start() - before)..(x.end() + after);
        input[range].fill(b' ');
    }

    let result2 = count(&regex_num, &input)?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
