use std::fs;

struct Password<'a> {
    data: &'a mut [u8],
}

impl<'a> Password<'a> {
    fn next(&mut self) -> &mut [u8] {
        let mut carry = 1;
        for x in self.data.iter_mut().rev() {
            if *x + carry <= b'z' {
                *x += carry;
                carry = 0;
            } else {
                *x = b'a';
                carry = 1;
            }
        }

        self.data
    }

    fn next_valid(&mut self) -> String {
        loop {
            let password = self.next();

            let mut iter = password
                .windows(2)
                .enumerate()
                .filter(|(_, x)| x[0] == x[1]);

            let check1 = iter
                .next()
                .and_then(|(pos1, _)| iter.last().map(|(pos2, _)| pos2 - pos1 > 1))
                .filter(|&x| x)
                .is_some();

            let check2 = password
                .windows(3)
                .any(|x| x[0] + 1 == x[1] && x[1] + 1 == x[2]);

            let check3 = !password
                .iter()
                .any(|&x| x == b'i' || x == b'o' || x == b'l');

            if check1 && check2 && check3 {
                return String::from_utf8_lossy(&password).into_owned();
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = fs::read_to_string("inputs/2015-day11.txt")?
        .trim()
        .as_bytes()
        .to_vec();

    let mut password_generator = Password { data: &mut input };

    let result1 = password_generator.next_valid();
    let result2 = password_generator.next_valid();

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
