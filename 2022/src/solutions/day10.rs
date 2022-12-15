use std::{io::{BufReader, BufRead}, fs::File};

#[derive(Clone, Copy)]
enum Instruction {
    NOOP,
    ADDX(i32)
}

impl Instruction {
    pub fn parse (line: &str) -> Instruction {
        let split: Vec<&str> = line.split(" ").collect();

        match split[0] {
            "noop" => Self::NOOP,
            "addx" => Self::ADDX(split[1].parse::<i32>().unwrap()),
            _ => panic!("Invalid instruction!"),
        }
    }

    pub fn getCycles (self) -> usize {
        match self {
            Self::NOOP => 1,
            Self::ADDX(_) => 2
        }
    }

    pub fn apply (self, curr: i32) -> i32 {
        match self {
            Self::NOOP => curr,
            Self::ADDX(x) => curr + x
        }
    }
}

struct CPU {
    regX: i32,
    nextRegX: i32,
    cycleNum: usize,
    sleepCycles: usize,
    instructions: Vec<Instruction>,
    instrPointer: usize
}

impl CPU {
    pub fn new (instructions: Vec<Instruction>) -> CPU {
        CPU { 
            regX: 1, 
            nextRegX: 1, 
            cycleNum: 0, 
            sleepCycles: 0, 
            instructions, 
            instrPointer: 0 
        }
    }

    pub fn simulateCycle (self: &mut Self) -> bool {
        if self.sleepCycles == 0 {
            self.regX = self.nextRegX;
            if self.instrPointer >= self.instructions.len() {
                return false;
            }

            let instr = self.instructions[self.instrPointer];
            self.nextRegX = instr.apply(self.regX);
            self.sleepCycles = instr.getCycles();
            self.instrPointer += 1;
        }

        self.cycleNum += 1;
        self.sleepCycles -= 1;

        /*if self.sleepCycles == 0 {
            self.regX = self.nextRegX;
            self.instrPointer += 1;
        }*/

        true
    }

    pub fn getCycleNum (self: &Self) -> usize {
        self.cycleNum
    }

    pub fn getRegValue (self: &Self) -> i32 {
        self.regX
    }
}

pub fn solve (file: BufReader<File>) {
    let mut cpu = CPU::new(file.lines().filter_map(Result::ok).map(|s| Instruction::parse(&s)).collect());

    let mut signalSum = 0;

    let mut display: String = String::with_capacity(240 + 6);

    while cpu.simulateCycle() && cpu.getCycleNum() < 241 {
        if cpu.getCycleNum() % 40 == 20 {
            //println!("Signal @ {}: {} ({})", cpu.getCycleNum(), cpu.getRegValue(), cpu.getCycleNum() as i32 * cpu.getRegValue());
            signalSum += cpu.getCycleNum() as i32 * cpu.getRegValue();
        }

        let cursor: isize = (cpu.getCycleNum() - 1) as isize % 40;

        if (cpu.getRegValue() - 1) as isize <= cursor && cursor <= (cpu.getRegValue() + 1) as isize {
            display.push('#');
        } else {
            display.push('.');
        }

        if cpu.getCycleNum() % 40 == 0 {
            display.push('\n');
        }
    }

    println!("Signal sum: {signalSum}");
    println!("Display: ");
    println!("{display}");
}