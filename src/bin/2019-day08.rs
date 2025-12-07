use aoc::*;

use itertools::Itertools;

use std::iter;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const LAYER_SIZE: usize = WIDTH * HEIGHT;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);
    let input = input.trim().as_bytes();

    let result1 = input
        .chunks_exact(LAYER_SIZE)
        .map(|layer| {
            let (mut count_0, mut count_1, mut count_2) = (0usize, 0usize, 0usize);
            for pixel in layer {
                match pixel {
                    b'0' => count_0 += 1,
                    b'1' => count_1 += 1,
                    b'2' => count_2 += 1,
                    _ => (),
                }
            }
            (count_0, count_1 * count_2)
        })
        .min_by_key(|&(count_0, _)| count_0)
        .map(|(_, product)| product)
        .value()?;

    let mut image = [b'-'; LAYER_SIZE];

    input.chunks_exact(LAYER_SIZE).for_each(|layer| {
        for (pixel, &layer_pixel) in image.iter_mut().zip(layer) {
            if *pixel == b'-' {
                match layer_pixel {
                    b'0' => *pixel = b' ',
                    b'1' => *pixel = b'#',
                    _ => *pixel = b'-',
                }
            }
        }
    });

    let result2 = String::from_utf8(
        image
            .chunks_exact(WIDTH)
            .flat_map(|row| iter::chain(row, b"\n").copied())
            .collect_vec(),
    )?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
