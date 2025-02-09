#![allow(unused)]

struct Cpu {
    registers: [u16; 16],
    memory: [u8; 4096],
    pc: u16,
}
impl Cpu {
    fn new() -> Cpu {
        Cpu {
            registers: [0; 16],
            memory: [0; 4096],
            pc: 0,
        }
    }

    fn fetch(&mut self) -> u16 {
        let opcode =
            (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16;
        self.pc += 2;
        opcode
    }

    fn decode(opcode: u16) -> Result<(&'static str, u8, u8), &'static str> {
        let op1 = (opcode >> 12) as u8;
        let dst = ((opcode >> 8) & 0x0F) as u8;
        match op1 {
            0b0000 => {
                let op2 = ((opcode >> 4) & 0x0F) as u8;
                let src = (opcode & 0x0F) as u8;
                match op2 {
                    0b1010 => Ok(("ADD", dst, src)),
                    0b0010 => Ok(("SUB", dst, src)),
                    0b1100 => Ok(("AND", dst, src)),
                    0b1110 => Ok(("OR", dst, src)),
                    0b1101 => Ok(("XOR", dst, src)),
                    0b1011 => Ok(("NOT", dst, src)),
                    _ => Err("Unknown operation"),
                }
            }
            0b0100 => {
                let imm = (opcode & 0xFF) as u8;
                Ok(("ADDI", dst, imm))
            }
            _ => Err("Unknown opcode"),
        }
    }

    fn step(&mut self) -> bool {
        let opcode = self.fetch();
        match Cpu::decode(opcode) {
            Ok((op, dst, src)) => {
                println!(
                    "{} R{} {}",
                    op,
                    dst,
                    if op == "ADDI" {
                        format!("#{:02x}", src)
                    } else {
                        format!("R{}", src)
                    }
                );
                match op {
                    "ADD" => {
                        self.registers[dst as usize] += self.registers[src as usize];
                    }
                    "ADDI" => {
                        let src = if src & 0x80 != 0 {
                            src as u16 | 0xFF00
                        } else {
                            src as u16
                        };
                        self.registers[dst as usize] += src;
                    }
                    "SUB" => {
                        self.registers[dst as usize] -= self.registers[src as usize];
                    }
                    "AND" => {
                        self.registers[dst as usize] &= self.registers[src as usize];
                    }
                    "OR" => {
                        self.registers[dst as usize] |= self.registers[src as usize];
                    }
                    "XOR" => {
                        self.registers[dst as usize] ^= self.registers[src as usize];
                    }
                    "NOT" => {
                        self.registers[dst as usize] = !self.registers[src as usize];
                    }
                    _ => unreachable!(),
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                return false;
            }
        }
        true
    }

    fn store_program(&mut self, program: &[u16]) {
        for (i, &word) in program.iter().enumerate() {
            self.memory[i * 2] = (word >> 8) as u8;
            self.memory[i * 2 + 1] = (word & 0xFF) as u8;
        }
    }

    fn set_register(&mut self, register: &[u16; 16]) {
        self.registers = *register;
    }
}
impl std::fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Registers:")?;
        for (i, r) in self.registers.iter().enumerate() {
            writeln!(f, "R{}: {:016b}", i, r)?;
        }
        writeln!(f, "Program Counter: {}", self.pc)?;
        Ok(())
    }
}

fn main() {
    println!("Addition");
    let mut cpu = Cpu::new();
    cpu.store_program(&[
        0x0020, // SUB R0, R0
        0x00A6, // ADD R0, R6
        0x00A7, // ADD R0, R7
        0x00A8, // ADD R0, R8
        0x00A9, // ADD R0, R9
        0x00AA, // ADD R0, R10
    ]);
    cpu.set_register(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0, 0, 0, 0, 0]);
    println!("{:?}", cpu);
    while cpu.step() {
        println!("{:?}", cpu);
    }

    println!("-----------------");
    println!("Bit Manipulation");
    let mut cpu = Cpu::new();
    cpu.store_program(&[
        0x0020, // SUB R0, R0
        0x0121, // SUB R1, R1
        0x0222, // SUB R2, R2
        0x0323, // SUB R3, R3
        0x405C, // ADDI R0, #0x5C
        0x4120, // ADDI R1, #0x20
        0x42BF, // ADDI R2, #0xBF
        0x4310, // ADDI R3, #0x10
        0x01E0, // OR R1, R0
        0x02C0, // AND R2, R0
        0x03D0, // XOR R3, R0
    ]);
    cpu.set_register(&[0; 16]);
    println!("{:?}", cpu);
    while cpu.step() {
        println!("{:?}", cpu);
    }

    println!("-----------------");
    println!("Fibonacci");
    let mut cpu = Cpu::new();
    cpu.store_program(&[
        0x0020, // SUB R0, R0
        0x0121, // SUB R1, R1
        0x4001, // ADDI R0, #1
        0x00A1, // ADD R0, R1
        0x01A0, // ADD R1, R0
        0x00A1, // ADD R0, R1
        0x01A0, // ADD R1, R0
        0x00A1, // ADD R0, R1
        0x01A0, // ADD R1, R0
        0x00A1, // ADD R0, R1
    ]);
    cpu.set_register(&[0; 16]);
    println!("{:?}", cpu);
    while cpu.step() {
        println!("{:?}", cpu);
    }
}
