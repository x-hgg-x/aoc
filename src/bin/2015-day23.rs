use std::fs;

enum Instruction {
    Half(usize),
    Triple(usize),
    Increment(usize),
    Jump(i32),
    JumpIfEven(usize, i32),
    JumpIfOne(usize, i32),
}

fn get_register(c: u8) -> usize {
    match c {
        b'a' => 0,
        b'b' => 1,
        other => panic!("unknown register: {}", other),
    }
}

fn run(instructions: &[Instruction], mut registers: [i32; 2]) -> [i32; 2] {
    let mut ip = 0;
    while (0..instructions.len()).contains(&(ip as usize)) {
        match instructions[ip as usize] {
            Instruction::Half(r) => registers[r] /= 2,
            Instruction::Triple(r) => registers[r] *= 3,
            Instruction::Increment(r) => registers[r] += 1,
            Instruction::Jump(offset) => {
                ip += offset;
                continue;
            }
            Instruction::JumpIfEven(r, offset) => {
                if registers[r] % 2 == 0 {
                    ip += offset;
                    continue;
                }
            }
            Instruction::JumpIfOne(r, offset) => {
                if registers[r] == 1 {
                    ip += offset;
                    continue;
                }
            }
        };
        ip += 1;
    }
    registers
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/2015-day23.txt")?;

    let instructions: Vec<_> = input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();

            match &bytes[..3] {
                b"hlf" => Instruction::Half(get_register(bytes[4])),
                b"tpl" => Instruction::Triple(get_register(bytes[4])),
                b"inc" => Instruction::Increment(get_register(bytes[4])),
                b"jmp" => Instruction::Jump(String::from_utf8_lossy(&bytes[4..]).parse().unwrap()),
                b"jie" => Instruction::JumpIfEven(
                    get_register(bytes[4]),
                    String::from_utf8_lossy(&bytes[7..]).parse().unwrap(),
                ),
                b"jio" => Instruction::JumpIfOne(
                    get_register(bytes[4]),
                    String::from_utf8_lossy(&bytes[7..]).parse().unwrap(),
                ),
                other => panic!("unknown instruction: {}", String::from_utf8_lossy(other)),
            }
        })
        .collect();

    let result1 = run(&instructions, [0, 0])[1];
    let result2 = run(&instructions, [1, 0])[1];

    println!("{}", result1);
    println!("{}", result2);
    Ok(())
}
