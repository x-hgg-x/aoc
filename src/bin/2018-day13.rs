use eyre::{bail, ensure, Result};
use num_complex::Complex;

use std::cmp::Ordering;
use std::collections::btree_map::{BTreeMap, Entry};
use std::fs;
use std::iter::repeat;

const TURN_LEFT: Complex<i64> = Complex::new(0, 1);
const NO_TURN: Complex<i64> = Complex::new(1, 0);
const TURN_RIGHT: Complex<i64> = Complex::new(0, -1);
const TURNS: [Complex<i64>; 3] = [TURN_LEFT, NO_TURN, TURN_RIGHT];

#[derive(Clone, Copy, PartialEq, Eq)]
struct CartPosition(Complex<i64>);

impl Ord for CartPosition {
    fn cmp(&self, other: &Self) -> Ordering {
        (-self.0.im, self.0.re).cmp(&(-other.0.im, other.0.re))
    }
}

impl PartialOrd for CartPosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Cart {
    direction: Complex<i64>,
    turn_index: usize,
}

enum Tile {
    Empty,
    HorizontalLine,
    VerticalLine,
    LeftCurve,
    RightCurve,
    Intersection,
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
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/2018-day13.txt")?;

    let width = input.lines().map(|line| line.len()).max().unwrap();
    let height = input.lines().count();

    let mut tiles = Vec::new();
    let mut carts = BTreeMap::new();

    input
        .lines()
        .map(|line| line.bytes().chain(repeat(b' ')).take(width))
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.enumerate().map(move |(column_index, x)| match x {
                b' ' => (Tile::Empty, None),
                b'-' => (Tile::HorizontalLine, None),
                b'|' => (Tile::VerticalLine, None),
                b'\\' => (Tile::LeftCurve, None),
                b'/' => (Tile::RightCurve, None),
                b'+' => (Tile::Intersection, None),
                _ => {
                    let position = Complex::new(column_index as i64, -(row_index as i64));
                    let turn_index = 0;
                    match x {
                        b'<' => (Tile::HorizontalLine, Some((CartPosition(position), Cart { direction: Complex::new(-1, 0), turn_index }))),
                        b'>' => (Tile::HorizontalLine, Some((CartPosition(position), Cart { direction: Complex::new(1, 0), turn_index }))),
                        b'^' => (Tile::VerticalLine, Some((CartPosition(position), Cart { direction: Complex::new(0, 1), turn_index }))),
                        b'v' => (Tile::VerticalLine, Some((CartPosition(position), Cart { direction: Complex::new(0, -1), turn_index }))),
                        _ => panic!("unknown tile"),
                    }
                }
            })
        })
        .for_each(|(tile, cart_data): (Tile, Option<(CartPosition, Cart)>)| {
            tiles.push(tile);

            if let Some((cart_position, cart)) = cart_data {
                carts.insert(cart_position, cart);
            }
        });

    let grid = Grid::new(width, height, tiles)?;

    let mut first_crash_position = None;
    let mut buf = Vec::with_capacity(carts.len());

    let last_position = loop {
        buf.clear();
        buf.extend(carts.keys().copied());

        for cart_position in &buf {
            if let Some((mut cart_position, mut cart)) = carts.remove_entry(cart_position) {
                cart_position.0 += cart.direction;

                let index = grid.get_index((-cart_position.0.im) as usize, cart_position.0.re as usize);
                match grid.tiles[index] {
                    Tile::HorizontalLine | Tile::VerticalLine => (),
                    Tile::LeftCurve if cart.direction.im != 0 => cart.direction *= TURN_LEFT,
                    Tile::LeftCurve if cart.direction.re != 0 => cart.direction *= TURN_RIGHT,
                    Tile::RightCurve if cart.direction.im != 0 => cart.direction *= TURN_RIGHT,
                    Tile::RightCurve if cart.direction.re != 0 => cart.direction *= TURN_LEFT,
                    Tile::Intersection => {
                        cart.direction *= TURNS[cart.turn_index];
                        cart.turn_index = (cart.turn_index + 1) % TURNS.len();
                    }
                    _ => bail!("unable to follow path at {:?}", cart_position.0),
                }

                match carts.entry(cart_position) {
                    Entry::Vacant(entry) => {
                        entry.insert(cart);
                    }
                    Entry::Occupied(entry) => {
                        entry.remove_entry();

                        if first_crash_position.is_none() {
                            first_crash_position = Some(cart_position.0);
                        }
                    }
                };
            }
        }

        if carts.len() == 1 {
            break carts.keys().last().map(|x| x.0);
        }
    };

    let (x1, y1) = first_crash_position.map(|x| (x.re, -x.im)).unwrap();
    let (x2, y2) = last_position.map(|x| (x.re, -x.im)).unwrap();

    let result1 = format!("{},{}", x1, y1);
    let result2 = format!("{},{}", x2, y2);

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
