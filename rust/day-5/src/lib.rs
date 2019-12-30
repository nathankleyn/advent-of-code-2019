use std::convert::TryFrom;
use std::collections::HashSet;

#[derive(Debug)]
struct Computer {
    pub memory: Vec<i32>,
    input: Vec<i32>,
    last_output: Option<i32>
}

#[allow(dead_code)]
impl Computer {
    fn new(raw_memory: &str, input: Vec<i32>) -> Result<Self, String> {
        let memory = raw_memory.split(',')
            .filter(|x| !x.is_empty())
            .map(|x| {
                x.trim().parse::<i32>()
                    .map_err(|e| format!("Failed to parse memory as i32: {}", e))
            })
            .collect::<Result<Vec<i32>, String>>()?;

        Ok(Computer {
            memory,
            input,
            last_output: None
        })
    }

    fn exec(&mut self) -> Option<i32> {
        let mut pointer = 0;

        loop {
            let opcode_with_param_modes = OpcodeWithParamModes::try_from(self.memory[pointer])
                .expect("Unexpected opcode encountered!");

            match opcode_with_param_modes.exec(self, pointer) {
                ExecResult::Success(next_pointer) => pointer = next_pointer,
                ExecResult::Halt => break,
                ExecResult::Failed(err) => panic!(err),
            }
        };

        self.last_output
    }

    fn read(&mut self) -> i32 {
        self.input.remove(0)
    }

    fn write(&mut self, value: i32) -> () {
        self.last_output = Some(value);
    }
}

#[derive(Debug)]
struct OpcodeWithParamModes {
    opcode: Opcode,
    param_modes: Vec<ParamMode>
}

impl TryFrom<i32> for OpcodeWithParamModes {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let opcode = Opcode::try_from(value % 100)?;
        let writable_params = opcode.writable_params();

        let first_param_mode = if writable_params.contains(&1) {
            ParamMode::ImmediateMode
        } else {
            ParamMode::try_from((value / 100) % 10)?
        };
        let second_param_mode = if writable_params.contains(&2) {
            ParamMode::ImmediateMode
        } else {
            ParamMode::try_from((value / 1000) % 10)?
        };
        let third_param_mode = if writable_params.contains(&3) {
            ParamMode::ImmediateMode
        } else {
            ParamMode::try_from((value / 100000) % 10)?
        };
        let param_modes = vec![first_param_mode, second_param_mode, third_param_mode];

        Ok(OpcodeWithParamModes {
            opcode,
            param_modes
        })
    }
}

#[derive(Debug)]
enum ExecResult<E> {
    Success(usize),
    Halt,
    Failed(E),
}

impl OpcodeWithParamModes {
    fn exec(&self, computer: &mut Computer, opcode_pos: usize) -> ExecResult<String> {
        let params: Params = self.extract_params(&computer.memory, opcode_pos)
            .expect("Unable to extract params!");

        let next_pointer = opcode_pos + self.opcode.num_params() + 1;
        let res = self.opcode.exec(params);

        match res {
            OpcodeResult::NoOp => ExecResult::Success(next_pointer),
            OpcodeResult::WriteToOutput(value) => {
                computer.write(value);
                ExecResult::Success(next_pointer)
            },
            OpcodeResult::WriteValueToMemory(value, to) => {
                computer.memory[to] = value;
                ExecResult::Success(next_pointer)
            },
            OpcodeResult::WriteInputToMemory(to) => {
                computer.memory[to] = computer.read();
                ExecResult::Success(next_pointer)
            },
            OpcodeResult::JumpTo(to) => ExecResult::Success(to),
            OpcodeResult::Halt => ExecResult::Halt,
            OpcodeResult::Failed(err) => ExecResult::Failed(err)
        }
    }

    fn extract_params(&self, memory: &Vec<i32>, opcode_pos: usize) -> Result<Params, String> {
        let num_params = self.opcode.num_params();

        let mut params = Vec::with_capacity(3);
        params.resize(3, None);

        for i in 0..num_params {
            let raw = memory[opcode_pos + i + 1];

            params[i] = match self.param_modes[i] {
                ParamMode::PositionMode => Some(memory[raw as usize]),
                ParamMode::ImmediateMode => Some(raw)
            };
        }

        Params::try_from(params)
    }
}

#[derive(Debug)]
enum ParamMode {
    PositionMode,
    ImmediateMode,
}

impl TryFrom<i32> for ParamMode {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ParamMode::PositionMode),
            1 => Ok(ParamMode::ImmediateMode),
            code => Err(format!("Unknown param mode '{}' encountered.", code))
        }
    }
}

struct Params {
    first: Option<i32>,
    second: Option<i32>,
    third: Option<i32>,
}

impl TryFrom<Vec<Option<i32>>> for Params {
    type Error = String;

    fn try_from(value: Vec<Option<i32>>) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err("Params can only be constructed from a Vec of exactly 3 in length.".to_string());
        }

        Ok(Params {
            first: value[0],
            second: value[1],
            third: value[2]
        })
    }
}

#[derive(Debug)]
enum Opcode {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    Halt = 99
}

impl TryFrom<i32> for Opcode {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Opcode::Add),
            2 => Ok(Opcode::Multiply),
            3 => Ok(Opcode::Input),
            4 => Ok(Opcode::Output),
            5 => Ok(Opcode::JumpIfTrue),
            6 => Ok(Opcode::JumpIfFalse),
            7 => Ok(Opcode::LessThan),
            8 => Ok(Opcode::Equals),
            99 => Ok(Opcode::Halt),
            code => Err(format!("Unknown opcode '{}' encountered.", code))
        }
    }
}

impl Opcode {
    fn num_params(&self) -> usize {
        match self {
            Opcode::Add => 3,
            Opcode::Multiply => 3,
            Opcode::Input => 1,
            Opcode::Output => 1,
            Opcode::JumpIfTrue => 2,
            Opcode::JumpIfFalse => 2,
            Opcode::LessThan => 3,
            Opcode::Equals => 3,
            Opcode::Halt => 0,
        }
    }


    fn writable_params(&self) -> HashSet<usize> {
        let mut params = HashSet::new();

        // Param indices are 1 based.
        match self {
            Opcode::Add => params.insert(3),
            Opcode::Multiply => params.insert(3),
            Opcode::Input => params.insert(1),
            Opcode::Output => false,
            Opcode::JumpIfTrue => false,
            Opcode::JumpIfFalse => false,
            Opcode::LessThan => params.insert(3),
            Opcode::Equals => params.insert(3),
            Opcode::Halt => false,
        };

        params
    }

    fn exec(&self, params: Params) -> OpcodeResult<String> {
        match self {
            Opcode::Add => {
                match (params.first, params.second, params.third) {
                    (Some(a), Some(b), Some(res)) => {
                        OpcodeResult::WriteValueToMemory(a + b, res as usize)
                    },
                    _ => OpcodeResult::Failed("Add expected 3 params.".to_string())
                }
            },
            Opcode::Multiply => {
                match (params.first, params.second, params.third) {
                    (Some(a), Some(b), Some(res)) => {
                        OpcodeResult::WriteValueToMemory(a * b, res as usize)
                    },
                    _ => OpcodeResult::Failed("Multiply expected 3 params.".to_string())
                }
            },
            Opcode::Input => {
                match params.first {
                    Some(res) => {
                        OpcodeResult::WriteInputToMemory(res as usize)
                    },
                    _ => OpcodeResult::Failed("Input expected 1 param.".to_string())
                }
            },
            Opcode::Output => {
                match params.first {
                    Some(value) => {
                        OpcodeResult::WriteToOutput(value)
                    },
                    _ => OpcodeResult::Failed("Output expected 1 param.".to_string())
                }
            },
            Opcode::JumpIfTrue => {
                match (params.first, params.second) {
                    (Some(value), Some(jump_to)) if value != 0 => {
                        OpcodeResult::JumpTo(jump_to as usize)
                    },
                    (Some(_), Some(_)) => {
                        OpcodeResult::NoOp
                    },
                    _ => OpcodeResult::Failed("JumpIfTrue expected 2 params.".to_string())
                }
            },
            Opcode::JumpIfFalse => {
                match (params.first, params.second) {
                    (Some(value), Some(jump_to)) if value == 0 => {
                        OpcodeResult::JumpTo(jump_to as usize)
                    },
                    (Some(_), Some(_)) => {
                        OpcodeResult::NoOp
                    },
                    _ => OpcodeResult::Failed("JumpIfFalse expected 2 params.".to_string())
                }
            },
            Opcode::LessThan => {
                match (params.first, params.second, params.third) {
                    (Some(a), Some(b), Some(res)) => {
                        OpcodeResult::WriteValueToMemory((a < b) as i32, res as usize)
                    },
                    _ => OpcodeResult::Failed("LessThan expected 3 params.".to_string())
                }
            },
            Opcode::Equals => {
                match (params.first, params.second, params.third) {
                    (Some(a), Some(b), Some(res)) => {
                        OpcodeResult::WriteValueToMemory((a == b) as i32, res as usize)
                    },
                    _ => OpcodeResult::Failed("Equals expected 3 params.".to_string())
                }
            },
            Opcode::Halt => OpcodeResult::Halt
        }
    }
}

#[derive(Debug)]
enum OpcodeResult<E> {
    NoOp,
    WriteToOutput(i32),
    WriteValueToMemory(i32, usize),
    WriteInputToMemory(usize),
    JumpTo(usize),
    Halt,
    Failed(E)
}

#[cfg(test)]
mod tests {
    use Computer;

    #[test]
    fn day_5_part_1_examples() {
        assert_eq!(Computer::new("3,0,4,0,99", vec![1]).unwrap().exec(), Some(1));
        assert_eq!(Computer::new("1002,4,3,4,33", vec![1]).unwrap().exec(), None);
    }

    #[test]
    fn day_5_part_1_test_input() {
        assert_eq!(Computer::new(include_str!("input"), vec![1]).unwrap().exec(), Some(9219874));
    }

    #[test]
    fn day_5_part_2_examples() {
        // check if equal to 8, positional mode
        assert_eq!(Computer::new("3,9,8,9,10,9,4,9,99,-1,8", vec![8]).unwrap().exec(), Some(1));
        assert_eq!(Computer::new("3,9,8,9,10,9,4,9,99,-1,8", vec![7]).unwrap().exec(), Some(0));

        // check if less than 8, positional mode
        assert_eq!(Computer::new("3,9,7,9,10,9,4,9,99,-1,8", vec![7]).unwrap().exec(), Some(1));
        assert_eq!(Computer::new("3,9,7,9,10,9,4,9,99,-1,8", vec![9]).unwrap().exec(), Some(0));

        // check if equal to 8, immediate mode
        assert_eq!(Computer::new("3,3,1108,-1,8,3,4,3,99", vec![8]).unwrap().exec(), Some(1));
        assert_eq!(Computer::new("3,3,1108,-1,8,3,4,3,99", vec![7]).unwrap().exec(), Some(0));

        // check if less than 8, immediate mode
        assert_eq!(Computer::new("3,3,1107,-1,8,3,4,3,99", vec![7]).unwrap().exec(), Some(1));
        assert_eq!(Computer::new("3,3,1107,-1,8,3,4,3,99", vec![9]).unwrap().exec(), Some(0));

        // jump, positional mode
        assert_eq!(Computer::new("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", vec![1]).unwrap().exec(), Some(1));
        assert_eq!(Computer::new("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", vec![0]).unwrap().exec(), Some(0));

        // jump, immediate mode
        assert_eq!(Computer::new("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", vec![1]).unwrap().exec(), Some(1));
        assert_eq!(Computer::new("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", vec![0]).unwrap().exec(), Some(0));

        // large example
        assert_eq!(Computer::new(include_str!("large_example"), vec![7]).unwrap().exec(), Some(999));
        assert_eq!(Computer::new(include_str!("large_example"), vec![8]).unwrap().exec(), Some(1000));
        assert_eq!(Computer::new(include_str!("large_example"), vec![9]).unwrap().exec(), Some(1001));
    }

    #[test]
    fn day_5_part_2_test_input() {
        assert_eq!(Computer::new(include_str!("input"), vec![5]).unwrap().exec(), Some(5893654));
    }
}
