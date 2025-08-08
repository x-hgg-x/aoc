use aoc::*;

use itertools::Itertools;

const NEIGHBORS: &[(i64, i64, i64); 6] = &[
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

struct Grid3d {
    tiles: Vec<u8>,
    dim_x: usize,
    dim_y: usize,
    dim_z: usize,
    min_x: i64,
    min_y: i64,
    min_z: i64,
}

impl Grid3d {
    fn new(bounds: Bounds) -> Result<Self> {
        let dim_x = (bounds.max_x - bounds.min_x + 1).try_into()?;
        let dim_y = (bounds.max_y - bounds.min_y + 1).try_into()?;
        let dim_z = (bounds.max_z - bounds.min_z + 1).try_into()?;
        let tiles = vec![0; dim_x * dim_y * dim_z];

        Ok(Self {
            tiles,
            dim_x,
            dim_y,
            dim_z,
            min_x: bounds.min_x,
            min_y: bounds.min_y,
            min_z: bounds.min_z,
        })
    }

    fn get_index(&self, x: i64, y: i64, z: i64) -> Result<usize> {
        let diff_x = usize::try_from(x - self.min_x)?;
        let diff_y = usize::try_from(y - self.min_y)?;
        let diff_z = usize::try_from(z - self.min_z)?;
        Ok(diff_z * (self.dim_x * self.dim_y) + diff_y * self.dim_x + diff_x)
    }

    fn tile(&self, x: i64, y: i64, z: i64) -> Result<u8> {
        let index = self.get_index(x, y, z)?;
        Ok(self.tiles[index])
    }

    fn tile_mut(&mut self, x: i64, y: i64, z: i64) -> Result<&mut u8> {
        let index = self.get_index(x, y, z)?;
        Ok(&mut self.tiles[index])
    }

    fn fill(&mut self, val: u8) -> Result<()> {
        let range_x = self.min_x..self.min_x + self.dim_x as i64;
        let range_y = self.min_y..self.min_y + self.dim_y as i64;
        let range_z = self.min_z..self.min_z + self.dim_z as i64;

        let mut queue = vec![(self.min_x, self.min_y, self.min_z)];

        while let Some((x, y, z)) = queue.pop() {
            *self.tile_mut(x, y, z)? = val;

            for &(dx, dy, dz) in NEIGHBORS {
                let (new_x, new_y, new_z) = (x + dx, y + dy, z + dz);

                if range_x.contains(&new_x)
                    && range_y.contains(&new_y)
                    && range_z.contains(&new_z)
                    && self.tile(new_x, new_y, new_z)? == 0
                {
                    queue.push((new_x, new_y, new_z));
                }
            }
        }

        Ok(())
    }
}

#[derive(Default)]
struct Bounds {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64,
}

fn count_sides(grid: &Grid3d, cubes: &[(i64, i64, i64)], val: u8) -> Result<usize> {
    cubes
        .iter()
        .map(|&(x, y, z)| {
            NEIGHBORS
                .iter()
                .map(|&(dx, dy, dz)| grid.tile(x + dx, y + dy, z + dz))
                .try_process(|iter| iter.filter(|&tile| tile == val).count())
        })
        .try_sum()
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let cubes: Vec<(i64, i64, i64)> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| Ok(x.parse()?))
                .try_process(|mut iter| iter.next_tuple())?
                .value()
        })
        .try_collect()?;

    let bounds = cubes.iter().fold(Bounds::default(), |bounds, cube| Bounds {
        min_x: bounds.min_x.min(cube.0 - 1),
        max_x: bounds.max_x.max(cube.0 + 1),
        min_y: bounds.min_y.min(cube.1 - 1),
        max_y: bounds.max_y.max(cube.1 + 1),
        min_z: bounds.min_z.min(cube.2 - 1),
        max_z: bounds.max_z.max(cube.2 + 1),
    });

    let mut grid = Grid3d::new(bounds)?;

    for &(x, y, z) in &cubes {
        *grid.tile_mut(x, y, z)? = u8::MAX;
    }

    let result1 = count_sides(&grid, &cubes, 0)?;

    grid.fill(1)?;
    let result2 = count_sides(&grid, &cubes, 1)?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
