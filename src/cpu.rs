//instructions
const  LDA: u8 = 0b0001;
const  ADD: u8 = 0b0010;
const  SUB: u8 = 0b0011;
const  STA: u8 = 0b0100;
const  LDI: u8 = 0b0101;
const  JMP: u8 = 0b0110;
const   JC: u8 = 0b0111;
const   JZ: u8 = 0b1000;
const  OUT: u8 = 0b1110;
const HALT: u8 = 0b1111;

// instruction microcode
const   NOP_M: [u16; 8] = [MI|CO, RO|II|CE, 0,     0,     0,           0, 0, 0];
const   LDA_M: [u16; 8] = [MI|CO, RO|II|CE, IO|MI, RO|AI, 0,           0, 0, 0];
const   ADD_M: [u16; 8] = [MI|CO, RO|II|CE, IO|MI, RO|BI, EO|AI|FI,    0, 0, 0];
const   SUB_M: [u16; 8] = [MI|CO, RO|II|CE, IO|MI, RO|BI, EO|AI|SU|FI, 0, 0, 0];
const   STA_M: [u16; 8] = [MI|CO, RO|II|CE, IO|MI, AO|RI, 0,           0, 0, 0];
const   LDI_M: [u16; 8] = [MI|CO, RO|II|CE, IO|AI, 0,     0,           0, 0, 0];
const   JMP_M: [u16; 8] = [MI|CO, RO|II|CE, IO|J,  0,     0,           0, 0, 0];
const   OUT_M: [u16; 8] = [MI|CO, RO|II|CE, AO|OI, 0,     0,           0, 0, 0];
const  HALT_M: [u16; 8] = [MI|CO, RO|II|CE, HLT,   0,     0,           0, 0, 0];

// control flag locations
const  FI: u16 = 0b1 <<  0;
const   J: u16 = 0b1 <<  1;
const  CO: u16 = 0b1 <<  2;
const  CE: u16 = 0b1 <<  3;
const  OI: u16 = 0b1 <<  4;
const  BI: u16 = 0b1 <<  5;
const  SU: u16 = 0b1 <<  6;
const  EO: u16 = 0b1 <<  7;
const  AO: u16 = 0b1 <<  8;
const  AI: u16 = 0b1 <<  9;
const  II: u16 = 0b1 << 10;
const  IO: u16 = 0b1 << 11;
const  RO: u16 = 0b1 << 12;
const  RI: u16 = 0b1 << 13;
const  MI: u16 = 0b1 << 14;
const HLT: u16 = 0b1 << 15;

pub struct Cpu {
    pub a: u8,
    pub b: u8,
    pub output: u8,
    pub pc: u8,
    pub ir: u8,
    pub ram: [u8; 16],
    pub mar: usize,
    pub cf: bool,
    pub zf: bool,
    pub microcode: Vec<u16>,
    pub step: usize,
    pub bus: u8,
    pub halted: bool
}

impl Cpu {
    pub fn new() -> Self {

        let microcode = [
            // zf = 0, cf = 0
             NOP_M, // 0000 - NOP
             LDA_M, // 0001 - LDA
             ADD_M, // 0010 - ADD
             SUB_M, // 0011 - SUB
             STA_M, // 0100 - STA
             LDI_M, // 0101 - LDI
             JMP_M, // 0110 - JMP
             NOP_M, // 0111 - JC
             NOP_M, // 1000 - JZ
             NOP_M, // 1001
             NOP_M, // 1010
             NOP_M, // 1011
             NOP_M, // 1100
             NOP_M, // 1101
             OUT_M, // 1110 - OUT
            HALT_M, // 1111 - HLT
            // zf = 0, cf = 1
             NOP_M, // 0000 - NOP
             LDA_M, // 0001 - LDA
             ADD_M, // 0010 - ADD
             SUB_M, // 0011 - SUB
             STA_M, // 0100 - STA
             LDI_M, // 0101 - LDI
             JMP_M, // 0110 - JMP
             JMP_M, // 0111 - JC
             NOP_M, // 1000 - JZ
             NOP_M, // 1001
             NOP_M, // 1010
             NOP_M, // 1011
             NOP_M, // 1100
             NOP_M, // 1101
             OUT_M, // 1110 - OUT
            HALT_M, // 1111 - HLT
            // zf = 1, cf = 0
             NOP_M, // 0000 - NOP
             LDA_M, // 0001 - LDA
             ADD_M, // 0010 - ADD
             SUB_M, // 0011 - SUB
             STA_M, // 0100 - STA
             LDI_M, // 0101 - LDI
             JMP_M, // 0110 - JMP
             NOP_M, // 0111 - JC
             JMP_M, // 1000 - JZ
             NOP_M, // 1001
             NOP_M, // 1010
             NOP_M, // 1011
             NOP_M, // 1100
             NOP_M, // 1101
             OUT_M, // 1110 - OUT
            HALT_M, // 1111 - HLT
            // zf = 1, cf = 1
             NOP_M, // 0000 - NOP
             LDA_M, // 0001 - LDA
             ADD_M, // 0010 - ADD
             SUB_M, // 0011 - SUB
             STA_M, // 0100 - STA
             LDI_M, // 0101 - LDI
             JMP_M, // 0110 - JMP
             JMP_M, // 0111 - JC
             JMP_M, // 1000 - JZ
             NOP_M, // 1001
             NOP_M, // 1010
             NOP_M, // 1011
             NOP_M, // 1100
             NOP_M, // 1101
             OUT_M, // 1110 - OUT
            HALT_M, // 1111 - HLT
        ].iter().flat_map(|x| x.iter().cloned()).collect();

        Self {
            a: 0,
            b: 0,
            output: 0,
            pc: 0,
            ir: 0,
            ram: [0; 16],
            mar: 0,
            cf: false,
            zf: false,
            microcode,
            step: 0,
            bus: 0,
            halted: false
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.b = 0;
        self.pc = 0;
        self.mar = 0;
        self.bus = 0;
        self.halted = false;
        self.output = 0;
        self.step = 0;
        self.cf = false;
        self.zf = false;
        self.ir = 0;

    }

    pub fn assemble(&mut self, code: &String) -> bool {

        let mut temp_ram = [0; 16];

        for (i, l) in code.lines().enumerate().take(16) {
            let tokens = l.split_ascii_whitespace().collect::<Vec<&str>>();

            if tokens.len() == 1 {
                if let Ok(num) = tokens[0].parse::<u8>() {
                    temp_ram[i] = num;
                } else {
                    temp_ram[i] = match tokens[0].to_ascii_uppercase().as_str() {
                        "OUT" => OUT << 4,
                        "HLT" => HALT << 4,
                        _ => return false
                    }
                }
            } else if tokens.len() == 2 {
                let opcode = match tokens[0].to_ascii_uppercase().as_str() {
                    "LDA" =>  LDA,
                    "ADD" =>  ADD,
                    "SUB" =>  SUB,
                    "STA" =>  STA,
                    "LDI" =>  LDI,
                    "JMP" =>  JMP,
                     "JC" =>   JC,
                     "JZ" =>   JZ,
                    "OUT" =>  OUT,
                    "HLT" => HALT,
                        _ => return false
                };

                let value = if let Ok(num) = tokens[1].parse::<u8>() {
                    num
                } else {
                    return false
                };

                temp_ram[i] = (opcode << 4) | value;
            } else if tokens.len() == 0 {
                temp_ram[i] = 0;
            } else {
                return false;
            }
        }

        self.ram = temp_ram;
        self.reset();

        return true;
    }

    pub fn get_micro_loc(&self) -> usize {
        let zf: usize = if self.zf {1} else {0};
        let cf: usize = if self.cf {1} else {0};

        let  flags = (zf << 8) | (cf << 7);
        let opcode = ((self.ir & 0xF0) >> 1) as usize;
        let   step = self.step & 0b111;

        let loc = flags | opcode | step;

        return loc
    }

    pub fn pulse(&mut self) {

        if !self.halted {

            self.bus = 0;

            let ctrl_word = self.microcode[self.get_micro_loc()];
            
            self.alu();

            if (ctrl_word & FI) != 0 {
                self.zf = self.alu() == 0;
                self.cf = self.alu() < self.a;
            }

            if (ctrl_word & CO) != 0 {
                self.bus = self.pc;
            }

            if (ctrl_word & EO) != 0 {
                self.bus = self.alu();
            }

            if (ctrl_word & AO) != 0 {
                self.bus = self.a;
            }

            if (ctrl_word & IO) != 0 {
                self.bus = self.ir & 0xF;
            }

            if (ctrl_word & RO) != 0 {
                self.bus = self.ram[self.mar];
            }

            if (ctrl_word & J) != 0 {
                self.pc = self.bus;
            }

            if (ctrl_word & CE) != 0 {
                self.pc = (self.pc + 1) & 0xF;
            }

            if (ctrl_word & OI) != 0 {
                self.output = self.bus;
            }

            if (ctrl_word & BI) != 0 {
                self.b = self.bus;
            }

            if (ctrl_word & AI) != 0 {
                self.a = self.bus;
            }

            if (ctrl_word & II) != 0 {
                self.ir = self.bus;
            }

            if (ctrl_word & RI) != 0 {
                self.ram[self.mar] = self.bus;
            }

            if (ctrl_word & MI) != 0 {
                self.mar = self.bus.into();
            }

            if (ctrl_word & HLT) != 0 {
                self.halted = true;
            }

            self.step = (self.step + 1) % 5;
        }
    }

    pub fn alu(&mut self) -> u8 {

        let rhs = if (self.microcode[self.get_micro_loc()] & SU) != 0 {
            (self.b^0xFF).wrapping_add(1)
        } else {
            self.b
        };
    
        self.a.wrapping_add(rhs)

    }
}