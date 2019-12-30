#[allow(dead_code)]
fn day_2_part_1(input: &str, noun: usize, verb: usize) -> Vec<usize> {
    let mut memory = parse(input);
    exec(&mut memory, noun, verb);
    memory
}

#[allow(dead_code)]
fn day_2_part_2(input: &str, target: usize) -> usize {
    let initial_memory = parse(input);

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut memory = initial_memory.clone();
            exec(&mut memory, noun, verb);

            if memory[0] == target {
                return 100 * noun + verb;
            }
        }
    }

    unimplemented!()
}

fn exec(memory: &mut Vec<usize>, noun: usize, verb: usize) {
    let mut pointer = 0;

    memory[1] = noun;
    memory[2] = verb;

    loop {
        let advance = match memory[pointer] {
            1 => {
                binay_instruction_exec(memory, pointer, BinaryInstruction::Add);
                4
            },
            2 => {
                binay_instruction_exec(memory, pointer, BinaryInstruction::Multiply);
                4
            },
            99 => break,
            _ => unimplemented!()
        };

        pointer += advance;
    };
}

enum BinaryInstruction {
    Add,
    Multiply
}

fn parse(input: &str) -> Vec<usize> {
    input.split(',')
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.trim().parse::<usize>().expect(&format!("Could not parse '{}' as usize.", x))
        })
        .collect()
}

fn binay_instruction_exec(memory: &mut Vec<usize>, pointer: usize, opcode: BinaryInstruction) {
    let x_addr = memory[pointer + 1];
    let y_addr = memory[pointer + 2];
    let output_addr = memory[pointer + 3];

    let x = memory[x_addr];
    let y = memory[y_addr];

    match opcode {
        BinaryInstruction::Add => memory[output_addr] = x + y,
        BinaryInstruction::Multiply => memory[output_addr] = x * y,
    }
}

#[cfg(test)]
mod tests {
    use super::day_2_part_1;
    use super::day_2_part_2;

    #[test]
    fn day_2_part_1_examples() {
        assert_eq!(day_2_part_1("1,0,0,0,99", 0, 0), vec![2,0,0,0,99]);
        assert_eq!(day_2_part_1("2,3,0,3,99", 3, 0), vec![2,3,0,6,99]);
        assert_eq!(day_2_part_1("2,4,4,5,99,0", 4, 4), vec![2,4,4,5,99,9801]);
        assert_eq!(day_2_part_1("1,1,1,4,99,5,6,0,99", 1, 1), vec![30,1,1,4,2,5,6,0,99]);
    }

    #[test]
    fn day_2_part_1_test_input() {
        assert_eq!(day_2_part_1(include_str!("input"), 12, 2)[0], 2890696);
    }

    #[test]
    fn day_2_part_2_test_input() {
        assert_eq!(day_2_part_2(include_str!("input"), 19690720), 8226);
    }
}
