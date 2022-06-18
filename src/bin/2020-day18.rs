use aoc::*;

use eyre::{bail, ensure};
use itertools::Itertools;

#[derive(Eq, PartialEq)]
enum Operation {
    Addition,
    Multiplication,
}

#[derive(Eq, PartialEq)]
enum Token {
    Number(u64),
    Operation(Operation),
    OpeningParenthesis,
    ClosingParenthesis,
}

impl Token {
    const ADDITION: Self = Token::Operation(Operation::Addition);
    const MULTIPLICATION: Self = Token::Operation(Operation::Multiplication);
}

struct State {
    operation: Option<Operation>,
    value: u64,
    reason: Option<Token>,
}

#[derive(Default)]
struct Stack(Vec<State>);

impl Stack {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn clear(&mut self) {
        self.0.clear();
    }

    fn push(&mut self, state: State) {
        self.0.push(state);
    }

    fn pop(&mut self) -> Option<State> {
        self.0.pop()
    }

    fn last_mut(&mut self) -> Option<&mut State> {
        self.0.last_mut()
    }

    fn merge(&mut self, value: u64) -> Result<()> {
        let last = self.last_mut().value()?;
        match last.operation {
            None => last.value = value,
            Some(Operation::Addition) => last.value += value,
            Some(Operation::Multiplication) => last.value *= value,
        }
        Ok(())
    }

    fn merge_addition(&mut self, value: u64, next_token: Option<&Token>) -> Result<()> {
        let last = self.last_mut().value()?;
        match last.operation {
            None => last.value = value,
            Some(Operation::Addition) => last.value += value,
            Some(Operation::Multiplication) => match next_token {
                Some(&Token::ADDITION) => self.push(State { operation: None, value, reason: Some(Token::ADDITION) }),
                _ => last.value *= value,
            },
        }
        Ok(())
    }

    fn pop_merge(&mut self) -> Result<()> {
        let value = self.pop().value()?.value;
        self.merge(value)
    }

    fn pop_merge_addition(&mut self, next_token: Option<&Token>) -> Result<()> {
        let value = self.pop().value()?.value;
        self.merge_addition(value, next_token)
    }

    fn pop_merge_after_addition(&mut self) -> Result<()> {
        if self.last_mut().value()?.reason == Some(Token::ADDITION) {
            self.pop_merge()?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let input = setup(file!())?;
    let input = String::from_utf8_lossy(&input);

    let expressions_tokens: Vec<_> = input
        .lines()
        .map(|line| {
            let mut tokens = Vec::new();
            let mut iter = line.bytes();

            while let Some(byte) = iter.next() {
                match byte {
                    b'0'..=b'9' => {
                        let value = iter.take_while_ref(|&x| x.is_ascii_digit()).fold((byte - b'0') as u64, |acc, x| 10 * acc + (x - b'0') as u64);
                        tokens.push(Token::Number(value));
                    }
                    b'+' => tokens.push(Token::ADDITION),
                    b'*' => tokens.push(Token::MULTIPLICATION),
                    b'(' => tokens.push(Token::OpeningParenthesis),
                    b')' => tokens.push(Token::ClosingParenthesis),
                    b' ' => (),
                    _ => bail!("invalid expression"),
                }
            }

            Ok(tokens)
        })
        .try_collect()?;

    let mut stack = Stack::default();

    let result1 = expressions_tokens
        .iter()
        .map(|tokens| {
            stack.clear();
            stack.push(State { operation: None, value: 0, reason: None });

            for token in tokens {
                match *token {
                    Token::Number(value) => stack.merge(value)?,
                    Token::ADDITION => stack.last_mut().value()?.operation = Some(Operation::Addition),
                    Token::MULTIPLICATION => stack.last_mut().value()?.operation = Some(Operation::Multiplication),
                    Token::OpeningParenthesis => stack.push(State { operation: None, value: 0, reason: Some(Token::OpeningParenthesis) }),
                    Token::ClosingParenthesis => stack.pop_merge()?,
                }
            }

            ensure!(stack.len() == 1, "operation failed");
            Ok(stack.pop().value()?.value)
        })
        .try_process(|iter| iter.sum::<u64>())?;

    let result2 = expressions_tokens
        .iter()
        .map(|tokens| {
            stack.clear();
            stack.push(State { operation: None, value: 0, reason: None });

            for (index, token) in tokens.iter().enumerate() {
                match *token {
                    Token::Number(value) => stack.merge_addition(value, tokens.get(index + 1))?,
                    Token::ADDITION => stack.last_mut().value()?.operation = Some(Operation::Addition),
                    Token::MULTIPLICATION => {
                        stack.pop_merge_after_addition()?;
                        stack.last_mut().value()?.operation = Some(Operation::Multiplication);
                    }
                    Token::OpeningParenthesis => stack.push(State { operation: None, value: 0, reason: Some(Token::OpeningParenthesis) }),
                    Token::ClosingParenthesis => {
                        stack.pop_merge_after_addition()?;
                        stack.pop_merge_addition(tokens.get(index + 1))?;
                    }
                }
            }

            stack.pop_merge_after_addition()?;

            ensure!(stack.len() == 1, "operation failed");
            Ok(stack.pop().value()?.value)
        })
        .try_process(|iter| iter.sum::<u64>())?;

    println!("{result1}");
    println!("{result2}");
    Ok(())
}
