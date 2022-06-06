use aoc::*;

use eyre::{bail, ensure};
use num_complex::Complex;

use std::iter::repeat;

const LEFT_TURN: Complex<i64> = Complex::new(0, 1);
const NO_TURN: Complex<i64> = Complex::new(1, 0);
const RIGHT_TURN: Complex<i64> = Complex::new(0, -1);
const TURNS: [Complex<i64>; 3] = [LEFT_TURN, NO_TURN, RIGHT_TURN];

struct Cart {
    direction: Complex<i64>,
    turn_index: usize,
}

enum Tile {
    Empty,
    HorizontalLine(Option<Cart>),
    VerticalLine(Option<Cart>),
    LeftCurve(Option<Cart>),
    RightCurve(Option<Cart>),
    Intersection(Option<Cart>),
}

struct Grid {
    width: usize,
    tiles: Vec<Tile>,
}

impl Grid {
    fn new(width: usize, height: usize, tiles: Vec<Tile>) -> Result<Self> {
        ensure!(width * height == tiles.len(), "unable to construct Grid: width * height != tiles.len()");
        Ok(Self { width, tiles })
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    fn get_position(&self, index: usize) -> (usize, usize) {
        let row = index / self.width;
        let column = index % self.width;
        (row, column)
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let width = input.lines().map(|line| line.len()).max().value()?;
    let height = input.lines().count();

    let mut tiles = Vec::with_capacity(width * height);
    let mut cart_indices = Vec::new();

    input.lines().map(|line| line.bytes().chain(repeat(b' ')).take(width)).enumerate().try_for_each(|(row_index, row)| {
        for (column_index, x) in row.enumerate() {
            match x {
                b' ' => tiles.push(Tile::Empty),
                b'-' => tiles.push(Tile::HorizontalLine(None)),
                b'|' => tiles.push(Tile::VerticalLine(None)),
                b'\\' => tiles.push(Tile::LeftCurve(None)),
                b'/' => tiles.push(Tile::RightCurve(None)),
                b'+' => tiles.push(Tile::Intersection(None)),
                b'<' => {
                    cart_indices.push(row_index * width + column_index);
                    tiles.push(Tile::HorizontalLine(Some(Cart { direction: Complex::new(-1, 0), turn_index: 0 })));
                }
                b'>' => {
                    cart_indices.push(row_index * width + column_index);
                    tiles.push(Tile::HorizontalLine(Some(Cart { direction: Complex::new(1, 0), turn_index: 0 })));
                }
                b'^' => {
                    cart_indices.push(row_index * width + column_index);
                    tiles.push(Tile::VerticalLine(Some(Cart { direction: Complex::new(0, 1), turn_index: 0 })));
                }
                b'v' => {
                    cart_indices.push(row_index * width + column_index);
                    tiles.push(Tile::VerticalLine(Some(Cart { direction: Complex::new(0, -1), turn_index: 0 })));
                }
                _ => bail!("unknown tile"),
            };
        }

        Ok(())
    })?;

    let mut grid = Grid::new(width, height, tiles)?;

    let mut first_crash_position = None;

    let last_position = loop {
        let mut collision = false;

        for cart_index in &mut cart_indices {
            let (mut cart_row, mut cart_column) = grid.get_position(*cart_index);

            let mut cart = match &mut grid.tiles[*cart_index] {
                Tile::Empty => bail!("empty tile at ({cart_row}, {cart_column})"),
                Tile::HorizontalLine(x) | Tile::VerticalLine(x) | Tile::LeftCurve(x) | Tile::RightCurve(x) | Tile::Intersection(x) => match x.take() {
                    Some(cart) => cart,
                    None => continue,
                },
            };

            cart_row = (cart_row as i64 - cart.direction.im) as usize;
            cart_column = (cart_column as i64 + cart.direction.re) as usize;
            *cart_index = grid.get_index(cart_row, cart_column);

            let new_tile_cart = match &mut grid.tiles[*cart_index] {
                Tile::Empty => bail!("empty tile at ({cart_row}, {cart_column})"),
                Tile::HorizontalLine(x) | Tile::VerticalLine(x) => x,
                Tile::LeftCurve(x) => {
                    if cart.direction.im != 0 {
                        cart.direction *= LEFT_TURN;
                    } else if cart.direction.re != 0 {
                        cart.direction *= RIGHT_TURN
                    } else {
                        bail!("unable to follow path at ({cart_row}, {cart_column})");
                    }
                    x
                }
                Tile::RightCurve(x) => {
                    if cart.direction.im != 0 {
                        cart.direction *= RIGHT_TURN;
                    } else if cart.direction.re != 0 {
                        cart.direction *= LEFT_TURN
                    } else {
                        bail!("unable to follow path at ({cart_row}, {cart_column})");
                    }
                    x
                }
                Tile::Intersection(x) => {
                    cart.direction *= TURNS[cart.turn_index];
                    cart.turn_index = (cart.turn_index + 1) % TURNS.len();
                    x
                }
            };

            match new_tile_cart {
                None => *new_tile_cart = Some(cart),
                _ => {
                    collision = true;
                    *new_tile_cart = None;
                    if first_crash_position.is_none() {
                        first_crash_position = Some((cart_row, cart_column));
                    }
                }
            };
        }

        if collision {
            cart_indices.retain(|&cart_index| match &grid.tiles[cart_index] {
                Tile::HorizontalLine(x) => x.is_some(),
                Tile::VerticalLine(x) => x.is_some(),
                Tile::LeftCurve(x) => x.is_some(),
                Tile::RightCurve(x) => x.is_some(),
                Tile::Intersection(x) => x.is_some(),
                Tile::Empty => false,
            });
        }

        cart_indices.sort_unstable();

        if cart_indices.len() == 1 {
            break grid.get_position(cart_indices[0]);
        }
    };

    let (x1, y1) = first_crash_position.map(|(row, column)| (column, row)).value()?;
    let (x2, y2) = (last_position.1, last_position.0);

    let result1 = format!("{x1},{y1}");
    let result2 = format!("{x2},{y2}");

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
