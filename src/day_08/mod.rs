use std::collections::HashSet;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::num::TryFromIntError;

#[derive(Clone, Debug)]
enum ErrorCode {
    IPOutOfBounds(usize),
    IntParseError,
    InstructionParseError,
    Terminated,
}

#[derive(Clone, Debug)]
pub struct ProgramError {
    msg: String,
    code: Option<ErrorCode>,
}

impl ProgramError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
            code: None,
        }
    }
}

impl From<ErrorCode> for ProgramError {
    fn from(code: ErrorCode) -> Self {
        ProgramError {
            msg: format!("{:?}", code),
            code: Some(code),
        }
    }
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for ProgramError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<TryFromIntError> for ProgramError {
    fn from(e: TryFromIntError) -> Self {
        ProgramError {
            msg: format!("couldn't parse: {:?}", e).to_string(),
            code: Some(ErrorCode::IntParseError),
        }
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl TryFrom<&str> for Instruction {
    type Error = ProgramError;

    fn try_from(string: &str) -> Result<Self, ProgramError> {
        let code = string.get(..3).ok_or(ErrorCode::InstructionParseError)?;
        let arg = string
            .get(4..)
            .ok_or(ErrorCode::IntParseError)?
            .parse::<isize>()
            .map_err(|_| ErrorCode::IntParseError)?;
        match code {
            "nop" => Ok(Instruction::Nop(arg)),
            "acc" => Ok(Instruction::Acc(arg)),
            "jmp" => Ok(Instruction::Jmp(arg)),
            _ => Err(ProgramError::new("unrecognized instruction")),
        }
    }
}

#[derive(Clone, Debug)]
struct Program {
    instructions: Vec<Instruction>,
    acc: isize,
    ip: usize,
    visited: HashSet<usize>,
    debug: bool,
}

impl TryFrom<&str> for Program {
    type Error = ProgramError;

    fn try_from(string: &str) -> Result<Self, ProgramError> {
        let mut instructions: Vec<Instruction> = Vec::new();
        for l in string.lines() {
            instructions.push(Instruction::try_from(l)?);
        }
        Ok(Program {
            instructions: instructions,
            acc: 0,
            ip: 0,
            visited: HashSet::new(),
            debug: false,
        })
    }
}

impl Program {
    fn step(&mut self) -> Result<(), ProgramError> {
        if self.ip == self.instructions.len() {
            return Err(ErrorCode::Terminated.into());
        }
        self.visited.insert(self.ip);
        let instruction = self
            .instructions
            .get(self.ip)
            .ok_or(ErrorCode::IPOutOfBounds(self.ip))?;
        if self.debug {
            println!("{}: {:?}", self.ip, instruction);
        }
        match instruction {
            Instruction::Nop(_) => {
                self.ip += 1;
            }
            Instruction::Acc(n) => {
                self.acc += n;
                self.ip += 1;
            }
            Instruction::Jmp(n) => {
                self.ip = usize::try_from(isize::try_from(self.ip)? as isize + n)?;
            }
        }
        Ok(())
    }

    fn acc_before_loop(&mut self) -> Result<isize, ProgramError> {
        while !self.visited.contains(&self.ip) {
            self.step()?;
        }
        Ok(self.acc)
    }
}

pub fn part_1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(Program::try_from(input.as_str())?
        .acc_before_loop()?
        .to_string())
}

pub fn part_2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let original_program = Program::try_from(input.as_str())?;
    for i in 0..original_program.instructions.len() {
        let mut p = original_program.clone();
        match p.instructions[i] {
            Instruction::Nop(n) => p.instructions[i] = Instruction::Jmp(n),
            Instruction::Jmp(n) => p.instructions[i] = Instruction::Nop(n),
            _ => continue,
        };
        match p.acc_before_loop() {
            Err(e) => match e.code {
                Some(ErrorCode::Terminated) => {
                    return Ok(p.acc.to_string());
                }
                _ => {}
            },
            _ => {}
        }
    }
    Err(Box::new(ProgramError::new("couldn't find answer")))
}

pub fn main(input: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(format!(
        "{}\n{}",
        part_1(input.clone())?,
        part_2(input.clone())?
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = concat!(
        "nop +0\n",
        "acc +1\n",
        "jmp +4\n",
        "acc +3\n",
        "jmp -3\n",
        "acc -99\n",
        "acc +1\n",
        "jmp -4\n",
        "acc +6\n",
    );

    #[test]
    fn test_program_loop_acc() -> Result<(), Box<dyn std::error::Error>> {
        let mut program = Program::try_from(TEST_INPUT)?;
        program.debug = true;
        assert_eq!(program.acc_before_loop()?, 5);
        Ok(())
    }

    #[test]
    fn test_part_2_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(part_2(TEST_INPUT.to_string())?.parse::<usize>()?, 8);
        Ok(())
    }
}
