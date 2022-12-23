use aoc::*;

use itertools::Itertools;

const SIZE: usize = 5;
const BOARD_SIZE: usize = SIZE * SIZE;

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let mut split = input.split_ascii_whitespace().peekable();

    let numbers: Vec<i8> = split.next().value()?.split(',').map(|x| x.parse()).try_collect()?;
    let sorted_numbers = numbers.iter().copied().enumerate().sorted_unstable_by_key(|&(_, x)| x).collect_vec();

    let mut boards = Vec::new();
    while split.peek().is_some() {
        let mut board = [0i8; BOARD_SIZE];
        for (num, elem) in split.by_ref().take(BOARD_SIZE).zip(&mut board) {
            *elem = num.parse()?;
        }
        boards.push(board);
    }

    let board_orders = boards
        .iter()
        .map(|board| {
            let mut board_order = [i8::MAX; BOARD_SIZE];
            for (elem, order) in board.iter().zip(&mut board_order) {
                if let Ok(index) = sorted_numbers.binary_search_by_key(elem, |&(_, x)| x) {
                    *order = sorted_numbers[index].0 as i8
                }
            }
            board_order
        })
        .collect_vec();

    let ((first, first_turn), (last, last_turn)) = board_orders
        .iter()
        .flat_map(|board_order| {
            let row_turn = board_order.chunks_exact(SIZE).map(|x| x.iter().copied().max()).min().flatten();
            let col_turn = (0..SIZE).map(|i_col| board_order.iter().copied().skip(i_col).step_by(SIZE).max()).min().flatten();
            row_turn.min(col_turn)
        })
        .enumerate()
        .minmax_by_key(|&(_, x)| x)
        .into_option()
        .value()?;

    let score = |index: usize, turn| {
        let unmarked = boards[index].iter().zip(&board_orders[index]).filter(|(_, &order)| order > turn).map(|(&elem, _)| elem as u64).sum::<u64>();
        let called_number = numbers[turn as usize] as u64;
        unmarked * called_number
    };

    let result1 = score(first, first_turn);
    let result2 = score(last, last_turn);

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
