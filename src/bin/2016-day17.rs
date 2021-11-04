use eyre::Result;
use std::fs;

#[derive(Clone, Default)]
struct State {
    position: (i8, i8),
    path: Vec<u8>,
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2016-day17.txt")?;
    let input = input.trim();

    const UP: (u8, (i8, i8)) = (b'U', (0, -1));
    const DOWN: (u8, (i8, i8)) = (b'D', (0, 1));
    const LEFT: (u8, (i8, i8)) = (b'L', (-1, 0));
    const RIGHT: (u8, (i8, i8)) = (b'R', (1, 0));
    const UDLR: [(u8, (i8, i8)); 4] = [UP, DOWN, LEFT, RIGHT];

    let mut buf = input.as_bytes().to_vec();
    let mut min_path = Option::<Vec<_>>::None;
    let mut max_path_len = 0;
    let mut states = vec![State::default()];

    while let Some(state) = states.pop() {
        if state.position == (3, 3) {
            let state_path_len = state.path.len();

            match &mut min_path {
                None => {
                    min_path = Some(state.path.clone());
                }
                Some(min_path) => {
                    if state_path_len < min_path.len() {
                        min_path.clone_from(&state.path);
                    }
                }
            }

            if state_path_len > max_path_len {
                max_path_len = state_path_len;
            }

            continue;
        }

        buf.truncate(input.len());
        buf.extend_from_slice(&state.path);
        let hash = md5::compute(&buf);
        let udlr_chars = [(hash[0] & 0xF0) >> 4, hash[0] & 0x0F, (hash[1] & 0xF0) >> 4, hash[1] & 0x0F];

        udlr_chars.iter().zip(UDLR).filter(|(&x, _)| x >= 11).for_each(|(_, (direction, step))| {
            let new_position = (state.position.0 + step.0, state.position.1 + step.1);

            if (0..4).contains(&new_position.0) && (0..4).contains(&new_position.1) {
                let mut new_state = state.clone();
                new_state.position = new_position;
                new_state.path.push(direction);
                states.push(new_state);
            }
        });
    }

    let result1 = String::from_utf8_lossy(min_path.as_ref().unwrap());
    let result2 = max_path_len;

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
