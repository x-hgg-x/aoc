use aoc::*;

use itertools::Itertools;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let points: Vec<(i64, i64, i64, i64)> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| Ok(x.parse()?))
                .try_process(|mut iter| iter.next_tuple())?
                .value()
        })
        .try_collect()?;

    let mut graph = vec![vec![]; points.len()];

    for (index_x, &(x0, x1, x2, x3)) in points.iter().enumerate() {
        for (index_y, &(y0, y1, y2, y3)) in points.iter().enumerate().skip(index_x + 1) {
            if ((y0 - x0).abs() + (y1 - x1).abs() + (y2 - x2).abs() + (y3 - x3).abs()) <= 3 {
                graph[index_x].push(index_y);
                graph[index_y].push(index_x);
            }
        }
    }

    let mut result = 0usize;
    let mut visited = vec![false; graph.len()];
    let mut queue = Vec::new();

    for index in 0..visited.len() {
        if !visited[index] {
            queue.clear();
            queue.push(index);

            while let Some(id) = queue.pop() {
                if !visited[id] {
                    visited[id] = true;
                    queue.extend(&graph[id]);
                }
            }
            result += 1;
        }
    }

    println!("{result}");
    Ok(())
}
