use regex::Regex;
use smallvec::SmallVec;

use std::fs;

#[derive(Clone)]
enum ValueType {
    Min,
    Max,
}

#[derive(Clone)]
enum Node {
    None,
    Bot(usize, ValueType),
    Output(usize, ValueType),
}

impl Default for Node {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Default)]
struct Bot {
    values: SmallVec<[i32; 2]>,
    outputs: SmallVec<[Node; 2]>,
}

impl Bot {
    fn get_value(&self, value_type: ValueType) -> i32 {
        *match value_type {
            ValueType::Min => self.values.iter().min().unwrap(),
            ValueType::Max => self.values.iter().max().unwrap(),
        }
    }
}

fn parse_node(node_type: &str, node_number: &str, value_type: ValueType) -> Node {
    match node_type {
        "bot" => Node::Bot(node_number.parse().unwrap(), value_type),
        "output" => Node::Output(node_number.parse().unwrap(), value_type),
        other => panic!("unknown node type: {}", other),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2016-day10.txt")?;

    let regex_bot = Regex::new(r#"(?m)^bot (\d+).*?(bot|output) (\d+).*?(bot|output) (\d+)$"#)?;
    let regex_value = Regex::new(r#"(?m)^value (\d+) goes to bot (\d+)$"#)?;

    let mut bot_instructions = Vec::new();
    let mut input_edges = Vec::new();

    for line in input.lines() {
        if let Some(cap) = regex_bot.captures(&line) {
            let bot_number: usize = cap[1].parse().unwrap();
            let node1 = parse_node(&cap[2], &cap[3], ValueType::Min);
            let node2 = parse_node(&cap[4], &cap[5], ValueType::Max);
            bot_instructions.push((bot_number, node1, node2));
        } else if let Some(cap) = regex_value.captures(&line) {
            let bot_number: usize = cap[2].parse().unwrap();
            let value = cap[1].parse().unwrap();
            input_edges.push((bot_number, value));
        } else {
            panic!("unknown instruction: {}", line)
        }
    }

    let mut bots = vec![Default::default(); bot_instructions.len()];
    for (bot_number, node1, node2) in bot_instructions {
        bots[bot_number] = Bot {
            values: SmallVec::new(),
            outputs: SmallVec::from_buf([node1, node2]),
        };
    }

    let mut processable_bots = Vec::with_capacity(input_edges.len());
    for (bot_number, value) in input_edges {
        let bot = &mut bots[bot_number];
        bot.values.push(value);
        if bot.values.len() == 2 {
            processable_bots.push(bot_number);
        }
    }

    let mut bot_61_17 = None;
    let mut output_0_1_2 = 1;

    while let Some(bot_number) = processable_bots.pop() {
        for output in bots[bot_number].outputs.clone() {
            match output {
                Node::Bot(output_bot_number, value_type) => {
                    let value = bots[bot_number].get_value(value_type);
                    let output_bot = &mut bots[output_bot_number];
                    output_bot.values.push(value);
                    if output_bot.values.len() == 2 {
                        processable_bots.push(output_bot_number);
                    }
                }
                Node::Output(0..=2, value_type) => {
                    output_0_1_2 *= bots[bot_number].get_value(value_type);
                }
                _ => {}
            }
        }

        if bot_61_17.is_none() {
            if let [17, 61] | [61, 17] = bots[bot_number].values[..] {
                bot_61_17 = Some(bot_number);
            }
        }
    }

    let result1 = bot_61_17.unwrap();
    let result2 = output_0_1_2;

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
