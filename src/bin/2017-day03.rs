use aoc::*;

use num_complex::Complex;

struct Memory {
    size: usize,
    values: Vec<i64>,
    current_position: Complex<usize>,
    line_direction: Complex<i64>,
    remaining_line_count: usize,
    line_len: usize,
}

impl Memory {
    fn new(half_size: usize) -> Self {
        let size = 2 * half_size + 1;

        let mut values = vec![0; size * size];
        values[half_size * (size + 1)] = 1;

        Self { size, values, current_position: Complex::new(half_size, half_size), line_direction: Complex::new(1, 0), remaining_line_count: 1, line_len: 1 }
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.size + column
    }

    fn next_index(&mut self) -> Result<usize> {
        self.current_position.re = usize::try_from(i64::try_from(self.current_position.re)? + self.line_direction.re)?;
        self.current_position.im = usize::try_from(i64::try_from(self.current_position.im)? + self.line_direction.im)?;
        self.remaining_line_count -= 1;

        if self.remaining_line_count == 0 {
            if self.line_direction.re == 0 {
                self.line_len += 1;
            }
            self.line_direction *= Complex::new(0, 1);
            self.remaining_line_count = self.line_len;
        }

        Ok(self.get_index(self.current_position.re, self.current_position.im))
    }

    fn neighbors_sum(&self) -> i64 {
        let (row, column) = (self.current_position.re, self.current_position.im);

        let rows = row.saturating_sub(1)..=(row + 1).min(self.size - 1);
        let columns = column.saturating_sub(1)..=(column + 1).min(self.size - 1);

        self.values.chunks_exact(self.size).skip(*rows.start()).take(rows.end() - rows.start() + 1).flat_map(|row| &row[columns.clone()]).sum()
    }
}

fn compute_steps(port: i64) -> i64 {
    if port == 1 {
        return 0;
    }

    let sqrt = ((port - 1) as f64).sqrt().floor() as i64;
    let port_diag_inf = sqrt.pow(2) + 1;

    if sqrt % 2 == 0 {
        sqrt / 2 + (sqrt / 2 - (port - port_diag_inf) % sqrt).abs()
    } else {
        let port_diag_sup = port_diag_inf + sqrt;
        if port < port_diag_sup {
            (sqrt + 1) / 2 + ((sqrt - 1) / 2 - (port - port_diag_inf)).abs()
        } else {
            (sqrt + 1) / 2 + ((sqrt + 1) / 2 - (port - port_diag_sup)).abs()
        }
    }
}

fn main() -> Result<()> {
    let input = String::from_utf8_lossy(&setup(file!())?).trim().parse()?;

    let result1 = compute_steps(input);

    let half_size = 1 + (input as f64).log(16.0).ceil() as usize;
    let mut memory = Memory::new(half_size);

    let result2 = loop {
        let index = memory.next_index()?;
        let value = memory.neighbors_sum();
        memory.values[index] = value;

        if value > input {
            break value;
        }
    };

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
