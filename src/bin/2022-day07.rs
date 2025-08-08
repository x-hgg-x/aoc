use aoc::*;

use eyre::bail;
use itertools::Itertools;

use std::cell::Cell;
use std::collections::HashMap;
use std::iter::repeat_n;

struct Dir {
    size: u64,
    children: Vec<String>,
    total_size: Cell<u64>,
}

impl Dir {
    fn total_size(&self, filesystem: &HashMap<String, Self>) -> u64 {
        let total_size = self.size + self.children.iter().map(|path| filesystem[path].total_size(filesystem)).sum::<u64>();
        self.total_size.set(total_size);
        total_size
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut filesystem = HashMap::new();
    let mut dir_stack = Vec::new();

    let mut lines_iter = input.lines();

    while let Some(command) = lines_iter.next() {
        if let Some(dir) = command.strip_prefix("$ cd ") {
            if dir == ".." {
                dir_stack.pop();
            } else if dir == "/" {
                dir_stack.push("root");
            } else {
                dir_stack.push(dir);
            }
        } else if command == "$ ls" {
            let path = repeat_n("/", dir_stack.len()).interleave(dir_stack.iter().copied()).collect::<String>();

            let mut size = 0;
            let mut children = Vec::new();

            for line in lines_iter.take_while_ref(|line| !line.starts_with('$')) {
                if let Some(dir) = line.strip_prefix("dir ") {
                    children.push(format!("{path}/{dir}"));
                } else {
                    size += line.split_ascii_whitespace().next().value()?.parse::<u64>()?;
                }
            }

            filesystem.insert(path, Dir { size, children, total_size: Cell::new(0) });
        } else {
            bail!("invalid command");
        }
    }

    let total_size = filesystem["/root"].total_size(&filesystem);
    let min_deleted_size = total_size - 40_000_000;

    let result1 = filesystem.values().map(|dir| dir.total_size.get()).filter(|&x| x < 100_000).sum::<u64>();
    let result2 = filesystem.values().map(|dir| dir.total_size.get()).filter(|&x| x > min_deleted_size).min().value()?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
