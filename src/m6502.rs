#![allow(non_snake_case)]

use std::u8;

use crate::memory::Memory;

pub const ADC_IMM: u8 = 0x69;
pub const ADC_ZPM: u8 = 0x65;
pub const ADC_ZPX: u8 = 0x75;
pub const ADC_ABS: u8 = 0x6d;
pub const ADC_ABX: u8 = 0x7d;
pub const ADC_ABY: u8 = 0x79;
pub const ADC_IDX: u8 = 0x61;
pub const ADC_IDY: u8 = 0x71;

pub const AND_IMM: u8 = 0x29;
pub const AND_ZPM: u8 = 0x25;
pub const AND_ZPX: u8 = 0x35;
pub const AND_ABS: u8 = 0x2d;
pub const AND_ABX: u8 = 0x3d;
pub const AND_ABY: u8 = 0x39;
pub const AND_IDX: u8 = 0x21;
pub const AND_IDY: u8 = 0x31;

pub const ASL_A: u8 = 0x0a;
pub const ASL_ZPM: u8 = 0x06;
pub const ASL_ZPX: u8 = 0x16;
pub const ASL_ABS: u8 = 0x0e;
pub const ASL_ABX: u8 = 0x1e;

//BIT

pub const BCC: u8 = 0x90;
pub const BCS: u8 = 0xb0;

pub const BEQ: u8 = 0xf0;
pub const BNE: u8 = 0xd0;

pub const BMI: u8 = 0x30;
pub const BPL: u8 = 0x10;

pub const BRK: u8 = 0x33; //remember op code change for (original is 0x00) null impl.
pub const RTI: u8 = 0x40;

pub const BVC: u8 = 0x50;
pub const BVS: u8 = 0x70;

//clears
pub const CLC: u8 = 0x18;
pub const CLD: u8 = 0xd8;
pub const CLI: u8 = 0x58;
pub const CLV: u8 = 0xb8;

pub const CPM_IMM: u8 = 0xc9;
pub const CPM_ZPM: u8 = 0xc5;
pub const CPM_ZPX: u8 = 0xd5;
pub const CPM_ABS: u8 = 0xcd;
pub const CPM_ABX: u8 = 0xdd;
pub const CPM_ABY: u8 = 0xd9;
pub const CPM_IDX: u8 = 0xc1;
pub const CPM_IDY: u8 = 0xd1;
pub const CPX_IMM: u8 = 0xe0;
pub const CPX_ZPM: u8 = 0xe4;
pub const CPX_ABS: u8 = 0xec;
pub const CPY_IMM: u8 = 0xc0;
pub const CPY_ZPM: u8 = 0xc4;
pub const CPY_ABS: u8 = 0xcc;

//DEC - eeprom
pub const DEX: u8 = 0xca;
pub const DEY: u8 = 0x88;
//INC - eeprom
pub const INX: u8 = 0xe8;
pub const INY: u8 = 0xc8;

pub const EOR_IMM: u8 = 0x49;
pub const EOR_ZPM: u8 = 0x45;
pub const EOR_ZPX: u8 = 0x55;
pub const EOR_ABS: u8 = 0x4d;
pub const EOR_ABX: u8 = 0x5d;
pub const EOR_ABY: u8 = 0x59;
pub const EOR_IDX: u8 = 0x41;
pub const EOR_IDY: u8 = 0x51;

pub const JMP_ABS: u8 = 0x4c;
pub const JMP_IND: u8 = 0x6c;
pub const JSR: u8 = 0x20;
pub const RTS: u8 = 0x60;

pub const LDA_IMM: u8 = 0xa9;
pub const LDA_ZPM: u8 = 0xa5;
pub const LDA_ZPX: u8 = 0xb5;
pub const LDA_ABS: u8 = 0xad;
pub const LDA_ABX: u8 = 0xbd;
pub const LDA_ABY: u8 = 0xb9;
pub const LDA_IDX: u8 = 0xa1;
pub const LDA_IDY: u8 = 0xb1;
pub const LDX_IMM: u8 = 0xa2;
pub const LDX_ZPM: u8 = 0xa6;
pub const LDX_ZPY: u8 = 0xb6;
pub const LDX_ABS: u8 = 0xae;
pub const LDX_ABY: u8 = 0xbe;
pub const LDY_IMM: u8 = 0xa0;
pub const LDY_ZPM: u8 = 0xa4;
pub const LDY_ZPX: u8 = 0xb4;
pub const LDY_ABS: u8 = 0xac;
pub const LDY_ABX: u8 = 0xbc;

pub const LSR: u8 = 0x4a;
//zpm, zpx, abs and abx manipulate mem, thus not used for the eeprom ver

pub const NOP: u8 = 0xea;

pub const ORA_IMM: u8 = 0x09;
pub const ORA_ZPM: u8 = 0x05;
pub const ORA_ZPX: u8 = 0x15;
pub const ORA_ABS: u8 = 0x0d;
pub const ORA_ABX: u8 = 0x1d;
pub const ORA_ABY: u8 = 0x19;
pub const ORA_IDX: u8 = 0x01;
pub const ORA_IDY: u8 = 0x11;

pub const SBC_IMM: u8 = 0xe9;
pub const SBC_ZPM: u8 = 0xe5;
pub const SBC_ZPX: u8 = 0xf5;
pub const SBC_ABS: u8 = 0xed;
pub const SBC_ABX: u8 = 0xfd;
pub const SBC_ABY: u8 = 0xf9;
pub const SBC_IDX: u8 = 0xe1;
pub const SBC_IDY: u8 = 0xf1;

pub const PHA: u8 = 0x48;
pub const PHP: u8 = 0x08;
pub const PLA: u8 = 0x68;
pub const PLP: u8 = 0x28;

pub const ROL_A: u8 = 0x2a;
pub const ROL_ZPM: u8 = 0x26;
pub const ROL_ZPX: u8 = 0x36;
pub const ROL_ABX: u8 = 0x2e;
pub const ROL_ABS: u8 = 0x3e;
pub const ROR_A: u8 = 0x6a;
pub const ROR_ZPM: u8 = 0x6a;
pub const ROR_ZPX: u8 = 0x66;
pub const ROR_ABS: u8 = 0x6e;
pub const ROR_ABX: u8 = 0x7e;

//zpm, zpx, abs and abx manipulate mem, thus not used for the eeprom ver

pub const SEC: u8 = 0x38;
pub const SED: u8 = 0xf8;
pub const SEI: u8 = 0x78;

pub const STA_ZPM: u8 = 0x85;
pub const STA_ZPX: u8 = 0x95;
pub const STA_ABS: u8 = 0x8d;
pub const STA_ABX: u8 = 0x9d;
pub const STA_ABY: u8 = 0x99;
pub const STA_IDX: u8 = 0x81;
pub const STA_IDY: u8 = 0x91;
pub const STX_ZPM: u8 = 0x86;
pub const STX_ZPY: u8 = 0x96;
pub const STX_ABS: u8 = 0x8e;
pub const STY_ZPM: u8 = 0x84;
pub const STY_ZPX: u8 = 0x94;
pub const STY_ABS: u8 = 0x8c;

pub const TAX: u8 = 0xaa;
pub const TAY: u8 = 0xa8;
pub const TSX: u8 = 0xba;
pub const TXA: u8 = 0x8a;
pub const TXS: u8 = 0x9a;
pub const TYA: u8 = 0x98;

pub struct CPU {
    a: u8,
    x: u8,
    y: u8,
    PC: u16, //program counter
    SP: u8,  //stack pointer
    data: u8,
    c: bool, //carry
    z: bool, //zero
    i: bool, //IRQB disable
    d: bool, //decimal
    b: bool, //BRK command
    //1
    v: bool, //overflow
    n: bool, //negative
    mem: Memory,
}
impl CPU {
    pub fn new(mem: Memory, start_loc: u16) -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            PC: start_loc,
            SP: 0,
            data: 0,
            c: false,
            z: false,
            i: false,
            d: false,
            b: false,
            v: false,
            n: false,
            mem: mem,
        }
    }
    /*
       Approaches:
           |V| 1. Open - instructions will not take arguments and will preform independintly of main.
           |X| 2. Closed - instructions will take an address as input and will change the PC acordingly. Main will need access to PC.
        need to setup all the appropriate addressing modes.
    */
    pub fn execute_instruction(&mut self) -> Option<i32> {
        // todo!("implement all ROL and ROR instuctions and fix");
        let mut cycles = 0;
        //read the op code
        self.data = self.mem.read(self.PC);
        if self.PC == 0xffff {
            self.PC = 0;
        }
        self.PC += 1;
        cycles += 1;
        // println!("{}", self.data);
        match self.data {
            LDA_IMM => {
                self.a = self.mem.read(self.PC);
                self.load_status(self.a);
            }
            LDA_ZPM => {
                self.a = self.mem.read(self.mem.read(self.PC) as u16); // a = mem[input] - searches for the value in the input loc.
                self.load_status(self.a);
            }
            LDA_ZPX => {
                let mut loc: u16 = self.mem.read(self.PC) as u16 + self.x as u16; //maybe should be done with add op
                cycles += 2;
                loc &= 0x00ff; //only get the zero-th page
                self.a = self.mem.read(loc);
                self.load_status(self.a);
            }
            LDA_ABS => {
                let mut loc: u16 = self.mem.read(self.PC) as u16;
                self.lda_absolute(&mut loc, &mut cycles);
            }
            LDA_ABX => {
                let mut loc: u16 = self.mem.read(self.PC) as u16 + self.x as u16;
                self.lda_absolute(&mut loc, &mut cycles);
            }
            LDA_ABY => {
                let mut loc: u16 = self.mem.read(self.PC) as u16 + self.y as u16;
                self.lda_absolute(&mut loc, &mut cycles);
            }
            LDA_IDX => {
                let loc = self.indirect_x(&mut cycles);
                self.a = self.mem.read(loc);
                self.load_status(self.a);
            }
            LDA_IDY => {
                let loc = self.indirect_y(&mut cycles);
                self.a = self.mem.read(loc);
                self.load_status(self.a);
            }
            ADC_IMM => {
                let loc = self.PC;
                self.adc_set_a(loc);
            }
            ADC_ZPM => {
                let loc = self.mem.read(self.PC) as u16;
                cycles += 1;
                self.adc_set_a(loc);
            }
            ADC_ZPX => {
                let mut loc = self.mem.read(self.PC) as u16 + self.x as u16;
                cycles += 2;
                loc &= 0x00FF;
                self.adc_set_a(loc);
            }
            ADC_ABS => {
                let mut loc = self.mem.read(self.PC) as u16;
                self.absolute(&mut loc, &mut cycles);
                self.adc_set_a(loc);
            }
            ADC_ABX => {
                let mut loc = self.mem.read(self.PC) as u16 + self.x as u16;
                self.absolute(&mut loc, &mut cycles);
                self.adc_set_a(loc);
            }
            ADC_ABY => {
                let mut loc = self.mem.read(self.PC) as u16 + self.y as u16;
                self.absolute(&mut loc, &mut cycles);
                self.adc_set_a(loc);
            }
            ADC_IDX => {
                let loc = self.indirect_x(&mut cycles);
                self.adc_set_a(loc);
            }
            ADC_IDY => {
                let loc = self.indirect_y(&mut cycles);
                self.adc_set_a(loc);
            }
            AND_IMM => {
                self.a &= self.mem.read(self.PC);
                //cycles happens at the end of function.
                self.and_status(self.a);
            }
            AND_ZPM => {
                self.a &= self.mem.read(self.mem.read(self.PC) as u16);
                cycles += 1;
                self.and_status(self.a);
            }
            AND_ZPX => {
                let mut loc = self.mem.read(self.PC) as u16 + self.x as u16;
                loc &= 0x00ff;
                self.a &= self.mem.read(loc);
                self.and_status(self.a);
            }
            AND_ABS => {
                let mut loc = self.mem.read(self.PC) as u16;
                self.and_absolute(self.a, &mut loc, &mut cycles);
            }
            AND_ABX => {
                let mut loc = self.mem.read(self.PC) as u16 + self.x as u16;
                self.and_absolute(self.a, &mut loc, &mut cycles);
            }
            AND_ABY => {
                let mut loc = self.mem.read(self.PC) as u16 + self.y as u16;
                self.and_absolute(self.a, &mut loc, &mut cycles);
            }
            AND_IDX => {
                let loc = self.indirect_x(&mut cycles);
                self.a &= self.mem.read(loc);
                self.and_status(self.a);
            }
            AND_IDY => {
                let loc = self.indirect_y(&mut cycles);
                self.a &= self.mem.read(loc);
                self.and_status(self.a);
            }
            ASL_A => {
                let c = (self.a >> 7) != 0;
                self.a <<= 1;
                self.asl_status(self.a, c);
                self.PC -= 1;
            }
            ASL_ZPM => {
                let loc = self.mem.read(self.PC);
                let value = self.mem.read(loc as u16);
                let c = (value >> 7) != 0;
                self.mem.write(loc as u16, value << 1);
                cycles += 2;
                self.asl_status(value, c);
                cycles += 1;
            }
            ASL_ZPX => {
                let loc = self.mem.read(self.PC) as u16 + self.x as u16;
                let value = self.mem.read(loc);
                let c = (value >> 7) != 0;
                self.mem.write(loc as u16, value << 1);
                cycles += 2;
                self.asl_status(value, c);
                cycles += 1;
            }
            ASL_ABS => {
                let mut loc = self.mem.read(self.PC) as u16;
                self.absolute(&mut loc, &mut cycles);
                let value = self.mem.read(loc);
                let c = (value >> 7) != 0;
                self.mem.write(loc as u16, value << 1);
                cycles += 2;
                self.asl_status(value, c);
            }
            ASL_ABX => {
                let mut loc = self.mem.read(self.PC) as u16 + self.x as u16;
                self.absolute(&mut loc, &mut cycles);
                let value = self.mem.read(loc);
                let c = (value >> 7) != 0;
                self.mem.write(loc as u16, value << 1);
                cycles += 2;
                self.asl_status(value, c);
            }
            BCC => {
                self.relative(&mut cycles, !self.c);
                return Some(cycles);
            }
            BCS => {
                self.relative(&mut cycles, self.c);
                return Some(cycles);
            }
            BEQ => {
                self.relative(&mut cycles, self.z);
                return Some(cycles);
            }
            BNE => {
                self.relative(&mut cycles, !self.z);
                return Some(cycles);
            }
            BMI => {
                self.relative(&mut cycles, self.n);
                return Some(cycles);
            }
            BPL => {
                self.relative(&mut cycles, !self.n);
                return Some(cycles);
            }
            BRK => {
                //interrupt - whatever that means.
                self.push_ret(&mut cycles);
                cycles += 1;
                let sr = self.status_reg();
                self.stack_push(sr, &mut cycles);
                self.i = true;
                cycles += 1;
            }
            BVC => {
                self.relative(&mut cycles, !self.n);
                return Some(cycles);
            }
            BVS => {
                self.relative(&mut cycles, self.n);
                return Some(cycles);
            }
            CLC => {
                self.PC -= 1; //cant overflow because PC+=1 at start.
                self.c = false;
            }
            CLD => {
                self.PC -= 1;
                self.d = false;
            }
            CLI => {
                self.PC -= 1;
                self.i = false;
            }
            CLV => {
                self.PC -= 1;
                self.v = false;
            }
            CPM_IMM => {
                self.compare_status((self.a as i16 - self.mem.read(self.PC) as i16) as u16);
            }
            CPM_ZPM => {
                let loc = self.mem.read(self.PC) as u16;
                cycles += 1;
                self.compare_status(self.a as u16 - self.mem.read(loc) as u16);
            }
            CPM_ZPX => {
                let mut loc = self.mem.read(self.PC) as u16 + self.x as u16;
                cycles += 2;
                loc &= 0x00ff;
                self.compare_status(self.a as u16 - self.mem.read(loc) as u16);
            }
            CPM_ABS => {
                let mut loc = self.mem.read(self.PC) as u16;
                self.absolute(&mut loc, &mut cycles);
                self.compare_status(self.a as u16 - self.mem.read(loc) as u16);
            }
            CPM_ABX => {
                let mut loc = self.mem.read(self.PC) as u16 + self.x as u16;
                self.absolute(&mut loc, &mut cycles);
                self.compare_status(self.a as u16 - self.mem.read(loc) as u16);
            }
            CPM_ABY => {
                let mut loc = self.mem.read(self.PC) as u16 + self.y as u16;
                self.absolute(&mut loc, &mut cycles);
                self.compare_status(self.a as u16 - self.mem.read(loc) as u16);
            }
            CPM_IDX => {
                let loc = self.indirect_x(&mut cycles);
                self.compare_status(self.a as u16 - self.mem.read(loc) as u16);
            }
            CPM_IDY => {
                let loc = self.indirect_y(&mut cycles);
                self.compare_status(self.a as u16 - self.mem.read(loc) as u16);
            }
            CPX_IMM => {
                self.compare_status(self.x as u16 - self.mem.read(self.PC) as u16);
            }
            CPX_ZPM => {
                let loc = self.mem.read(self.PC) as u16;
                self.compare_status(self.x as u16 - self.mem.read(loc) as u16);
            }
            CPX_ABS => {
                let mut loc = self.mem.read(self.PC) as u16;
                self.absolute(&mut loc, &mut cycles);
                self.compare_status(self.x as u16 - self.mem.read(loc) as u16);
            }
            CPY_IMM => {
                self.compare_status(self.y as u16 - self.mem.read(self.PC) as u16);
            }
            CPY_ZPM => {
                let loc = self.mem.read(self.PC) as u16;
                cycles += 1;
                self.compare_status(self.y as u16 - self.mem.read(loc) as u16);
            }
            CPY_ABS => {
                let mut loc = self.mem.read(self.PC) as u16;
                self.absolute(&mut loc, &mut cycles);
                self.compare_status(self.y as u16 - self.mem.read(loc) as u16);
            }
            DEX => {
                self.PC -= 1;
                if self.x == 0 {
                    self.x = 0xff;
                } else {
                    self.x -= 1;
                }
                self.dec_inc_status(self.x);
            }
            DEY => {
                self.PC -= 1;
                if self.y == 0 {
                    self.y = 0xff;
                } else {
                    self.y -= 1;
                }
                self.dec_inc_status(self.y);
            }
            INX => {
                self.PC -= 1;
                if self.x == 0xff {
                    self.x = 0;
                } else {
                    self.x += 1;
                }
                self.dec_inc_status(self.x);
            }
            INY => {
                self.PC -= 1;
                if self.y == 0xff {
                    self.y = 0;
                } else {
                    self.y += 1;
                }
                self.dec_inc_status(self.y);
            }
            EOR_IMM => {
                self.a ^= self.mem.read(self.PC);
                self.eor_status();
            }
            EOR_ZPM => {
                let loc = self.mem.read(self.PC) as u16;
                cycles += 1;
                self.a ^= self.mem.read(loc);
                self.eor_status();
            }
            EOR_ZPX => {
                let mut loc = self.mem.read(self.PC) as u16 + self.x as u16;
                cycles += 2;
                loc &= 0x00ff;
                self.a ^= self.mem.read(loc);
                self.eor_status();
            }
            EOR_ABS => {
                let mut loc = self.mem.read(self.PC) as u16;
                self.absolute(&mut loc, &mut cycles);
                self.a ^= self.mem.read(loc);
                self.eor_status();
            }
            EOR_ABX => {
                let mut loc = self.mem.read(self.PC) as u16 + self.x as u16;
                self.absolute(&mut loc, &mut cycles);
                self.a ^= self.mem.read(loc);
                self.eor_status();
            }
            EOR_ABY => {
                let mut loc = self.mem.read(self.PC) as u16 + self.y as u16;
                self.absolute(&mut loc, &mut cycles);
                self.a ^= self.mem.read(loc);
                self.eor_status();
            }
            EOR_IDX => {
                let loc = self.indirect_x(&mut cycles);
                self.a ^= self.mem.read(loc);
                self.eor_status();
            }
            EOR_IDY => {
                let loc = self.indirect_y(&mut cycles);
                self.a ^= self.mem.read(loc);
                self.eor_status();
            }
            JMP_ABS => {
                self.jump_absolute(&mut cycles);
                return Some(cycles);
            }
            JMP_IND => {
                //jmp wizardry
                let loc = self.indirect(&mut cycles);
                self.PC = loc;
                return Some(cycles);
            }
            JSR => {
                self.push_ret(&mut cycles);
                self.jump_absolute(&mut cycles);
                cycles += 1;
                return Some(cycles);
            }
            RTS => {
                let mut ret_address = self.stack_pop(&mut cycles) as u16;
                ret_address |= (self.stack_pop(&mut cycles) as u16) << 8;
                self.PC = ret_address;
                cycles += 1;
                return Some(cycles);
            }
            LDX_IMM => {
                self.x = self.mem.read(self.PC);
                self.load_status(self.x);
            }
            LDX_ZPM => {
                let loc = self.mem.read(self.PC) as u16;
                cycles += 1;
                self.x = self.mem.read(loc);
                self.load_status(self.x);
            }
            LDX_ZPY => {
                let mut loc = self.mem.read(self.PC) as u16 + self.y as u16;
                cycles += 2;
                loc &= 0x00ff;
                self.x = self.mem.read(loc);
                self.load_status(self.x);
            }
            LDX_ABS => {
                let mut loc = self.mem.read(self.PC) as u16;
                self.absolute(&mut loc, &mut cycles);
                self.x = self.mem.read(loc);
                self.load_status(self.x);
            }
            LDX_ABY => {
                let mut loc = self.mem.read(self.PC) as u16 + self.y as u16;
                self.absolute(&mut loc, &mut cycles);
                self.x = self.mem.read(loc);
                self.load_status(self.x);
            }
            LDY_IMM => {
                self.y = self.mem.read(self.PC);
                self.load_status(self.y);
            }
            LDY_ZPM => {
                let loc = self.mem.read(self.PC) as u16;
                cycles += 1;
                self.y = self.mem.read(loc);
                self.load_status(self.y);
            }
            LDY_ZPX => {
                let mut loc = self.mem.read(self.PC) as u16 + self.x as u16;
                cycles += 2;
                loc &= 0x00ff;
                self.y = self.mem.read(loc);
                self.load_status(self.y);
            }
            LDY_ABS => {
                let mut loc = self.mem.read(self.PC) as u16;
                self.absolute(&mut loc, &mut cycles);
                self.y = self.mem.read(loc);
                self.load_status(self.y);
            }
            LDY_ABX => {
                let mut loc = self.mem.read(self.PC) as u16 + self.x as u16;
                self.absolute(&mut loc, &mut cycles);
                self.y = self.mem.read(loc);
                self.load_status(self.y);
            }
            LSR => {
                let c = self.a >> 7 != 0;
                self.a >>= 1;
                self.lsr_status(c);
            }
            NOP => {
                //so it wont jump 2 addresses
                self.PC -= 1;
            }
            ORA_IMM => {
                self.a |= self.mem.read(self.PC);
                self.ora_status();
            }
            ORA_ZPM => {
                let loc = self.mem.read(self.PC) as u16;
                cycles += 1;
                self.a |= self.mem.read(loc);
                self.ora_status();
            }
            ORA_ZPX => {
                let mut loc = self.mem.read(self.PC) as u16 + self.x as u16;
                cycles += 2;
                loc &= 0x00ff;
                self.a |= self.mem.read(loc);
                self.ora_status();
            }
            ORA_ABS => {
                let mut loc = self.mem.read(self.PC) as u16;
                self.absolute(&mut loc, &mut cycles);
                self.a |= self.mem.read(loc);
                self.ora_status();
            }
            ORA_ABX => {
                let mut loc = self.mem.read(self.PC) as u16 + self.x as u16;
                self.absolute(&mut loc, &mut cycles);
                self.a |= self.mem.read(loc);
                self.ora_status();
            }
            ORA_ABY => {
                let mut loc = self.mem.read(self.PC) as u16 + self.y as u16;
                self.absolute(&mut loc, &mut cycles);
                self.a |= self.mem.read(loc);
                self.ora_status();
            }
            ORA_IDX => {
                let loc = self.indirect_x(&mut cycles);
                self.a |= self.mem.read(loc);
                self.ora_status();
            }
            ORA_IDY => {
                let loc = self.indirect_y(&mut cycles);
                self.a |= self.mem.read(loc);
                self.ora_status();
            }
            PHA => {
                self.stack_push(self.a, &mut cycles);
                self.PC -= 1;
            }
            PHP => {
                self.b = true;
                let sr = self.status_reg();
                self.stack_push(sr, &mut cycles);
                self.PC -= 1;
            }
            PLA => {
                self.a = self.stack_pop(&mut cycles);
                self.PC -= 1;
            }
            PLP => {
                let sr = self.stack_pop(&mut cycles);
                self.set_status_reg(sr);
            }
            ROL_A => {
                let last_bit = self.a >> 7 != 0;
                let c = self.a << 7 != 0;
                self.a <<= 1;
                self.a |= last_bit as u8;
                self.rotate_status(c);
                self.PC -= 1;
            }
            ROR_A => {
                //128 is last bit in binary
                let first_bit = self.a & 1 != 0;
                let c = self.a << 7 != 0;
                self.a >>= 1;
                self.a |= (first_bit as u8) << 7;
                self.rotate_status(c);
                self.PC -= 1;
            }
            RTI => {
                //takes 6.
                cycles -= 1;
                let sr = self.stack_pop(&mut cycles);
                self.set_status_reg(sr);
                let mut ret_address = self.stack_pop(&mut cycles) as u16;
                ret_address |= (self.stack_pop(&mut cycles) as u16) << 8;
                self.PC = ret_address;
                return Some(cycles);
            }
            STA_ZPM => {
                self.mem.write(self.PC, self.a);
                cycles += 1;
            }
            STA_ZPX => {
                let loc = (self.mem.read(self.PC) as u16 + self.x as u16) & 0x00ff;
                cycles += 1;
                self.mem.write(loc, self.a);
                cycles += 1;
            }
            STA_ABS => {
                let mut loc = self.mem.read(self.PC) as u16;
                self.absolute(&mut loc, &mut cycles);
                self.mem.write(loc, self.a);
                cycles += 1;
            }
            STA_ABX => {
                let mut loc = self.mem.read(self.PC) as u16 + self.x as u16;
                self.absolute(&mut loc, &mut cycles);
                self.mem.write(loc, self.a);
                cycles += 1;
            }
            STA_ABY => {
                let mut loc = self.mem.read(self.PC) as u16 + self.y as u16;
                self.absolute(&mut loc, &mut cycles);
                self.mem.write(loc, self.a);
                cycles += 1;
            }
            STA_IDX => {
                let loc = self.indirect_x(&mut cycles);
                self.mem.write(loc, self.a);
                cycles += 1;
            }
            STA_IDY => {
                let loc = self.indirect_y(&mut cycles);
                self.mem.write(loc, self.a);
                cycles += 1;
            }
            STX_ZPM => {
                self.mem.write(self.PC, self.x);
                cycles += 1;
            }
            STX_ZPY => {
                let loc = (self.mem.read(self.PC) as u16 + self.y as u16) & 0x00ff;
                cycles += 1;
                self.mem.write(loc, self.x);
                cycles += 1;
            }
            STX_ABS => {
                let mut loc = self.mem.read(self.PC) as u16;
                self.absolute(&mut loc, &mut cycles);
                self.mem.write(loc, self.x);
                cycles += 1;
            }
            STY_ZPM => {
                self.mem.write(self.PC, self.y);
                cycles += 1;
            }
            STY_ZPX => {
                let loc = (self.mem.read(self.PC) as u16 + self.x as u16) & 0x00ff;
                cycles += 1;
                self.mem.write(loc, self.y);
                cycles += 1;
            }
            STY_ABS => {
                let mut loc = self.mem.read(self.PC) as u16;
                self.absolute(&mut loc, &mut cycles);
                self.mem.write(loc, self.y);
                cycles += 1;
            }
            SBC_IMM => {
                self.sbc_set_a(self.PC);
            }
            SBC_ZPM => {
                let loc = self.mem.read(self.PC) as u16;
                cycles += 1;
                self.sbc_set_a(loc);
            }
            SBC_ZPX => {
                let mut loc = self.mem.read(self.PC) as u16 + self.x as u16;
                cycles += 2;
                loc &= 0x00ff;
                self.sbc_set_a(loc);
            }
            SBC_ABS => {
                let mut loc = self.mem.read(self.PC) as u16;
                self.absolute(&mut loc, &mut cycles);
                self.sbc_set_a(loc);
            }
            SBC_ABX => {
                let mut loc = self.mem.read(self.PC) as u16 + self.x as u16;
                self.absolute(&mut loc, &mut cycles);
                self.sbc_set_a(loc);
            }
            SBC_ABY => {
                let mut loc = self.mem.read(self.PC) as u16 + self.y as u16;
                self.absolute(&mut loc, &mut cycles);
                self.sbc_set_a(loc);
            }
            SBC_IDX => {
                let loc = self.indirect_x(&mut cycles);
                self.sbc_set_a(loc);
            }
            SBC_IDY => {
                let loc = self.indirect_y(&mut cycles);
                self.sbc_set_a(loc);
            }
            SEC => {
                self.c = true;
                self.PC -= 1;
            }
            SED => {
                self.d = true;
                self.PC -= 1;
            }
            SEI => {
                self.i = true;
                self.PC -= 1;
            }
            TAX => {
                self.x = self.a;
                self.PC -= 1;
                self.transfer_status(self.x);
            }
            TAY => {
                self.y = self.a;
                self.PC -= 1;
                self.transfer_status(self.y);
            }
            TSX => {
                self.x = self.SP;
                self.PC -= 1;
                self.transfer_status(self.x);
            }
            TXA => {
                self.a = self.x;
                self.PC -= 1;
                self.transfer_status(self.a);
            }
            TXS => {
                self.SP = self.x;
                self.PC -= 1;
                //status is not checked for stack pointer.
            }
            TYA => {
                self.a = self.y;
                self.PC -= 1;
                self.transfer_status(self.a);
            }

            _ => {
                self.status_reg();
                return None;
            }
        }
        // println!("accumulator {}, x {}, y {}", self.a, self.x, self.y);
        // println!("negative {}, zero {}", self.n, self.z);
        self.PC += 1;
        cycles += 1;
        Some(cycles)
    }

    //abstract general addressing modes. Only change the loc.
    fn absolute(&mut self, loc: &mut u16, cycles: &mut i32) {
        *cycles += 1;
        //for crossing page bounderies.
        self.PC += 1;
        let part1 = self.mem.read(self.PC) as u16;
        if *loc > 255 {
            *cycles += 1;
        }
        *cycles += 1;
        *loc |= part1 << 8;
    }
    //this returns the loc instead of mutating because the input is constant. So there is no need for it.
    fn indirect(&mut self, cycles: &mut i32) -> u16 {
        let mut loc: u16 = self.mem.read(self.PC) as u16;
        self.absolute(&mut loc, cycles);
        loc = (self.mem.read(loc) as u16) | (self.mem.read(loc + 1) as u16) << 8;
        *cycles += 1;
        //when crossing page bounderies jmp doesnt add cycles because of magic.
        if *cycles < 5 {
            *cycles += 1;
        }
        loc
    }
    fn indirect_x(&mut self, cycles: &mut i32) -> u16 {
        let mut loc: u16 = self.mem.read(self.PC) as u16 + self.x as u16;
        *cycles += 2;
        loc &= 0x00ff;
        //lookup with next mem slot and add to find mem loc.
        loc = (self.mem.read(loc) as u16) | (self.mem.read(loc + 1) as u16) << 8;
        *cycles += 2;
        loc
    }
    fn indirect_y(&mut self, cycles: &mut i32) -> u16 {
        let mut loc: u16 = self.mem.read(self.PC) as u16;
        *cycles += 1;
        //lookup with next mem slot and add to find mem loc. But with post addition of y.
        let part1 = (self.mem.read(loc) as u16) + self.y as u16;
        *cycles += 2;
        if part1 > 255 {
            *cycles += 1;
        }
        loc = part1 | (self.mem.read(loc + 1) as u16) << 8;
        loc
    }
    fn relative(&mut self, cycles: &mut i32, condition: bool) {
        if condition {
            // println!("PC start {}", self.PC);
            let branch_offset = self.mem.read(self.PC);
            //if greater than 127 it is negative.
            if branch_offset > 127 {
                self.PC -= (255 - branch_offset) as u16;
            } else {
                self.PC += self.mem.read(self.PC) as u16 + 1; //+1 because the offset is after the 2 byte instruction but PC += 1 in start of instuction
            }
            *cycles += 2;
            //so it wont add another.
        } else {
            self.PC += 1;
            *cycles += 1;
        }
    }

    //utilities for enforcing cpu bounderies
    fn stack_pop(&mut self, cycles: &mut i32) -> u8 {
        if self.SP == 0 {
            self.SP = 0xff;
        } else {
            self.SP -= 1;
        }
        let out = self.mem.read(self.SP as u16 + 0x100);
        *cycles += 1;
        //pop
        self.mem.write(self.SP as u16 + 0x100, 0);
        *cycles += 1;
        out
    }
    fn stack_push(&mut self, data: u8, cycles: &mut i32) {
        self.mem.write(self.SP as u16 + 0x100, data);
        if self.SP == 0xff {
            self.SP = 0;
        } else {
            self.SP += 1;
        }
        *cycles += 1;
    }
    //evil fucking fuction (dunno how to improve)
    fn status_reg(&mut self) -> u8 {
        let mut sr: u8 = self.c as u8;
        sr |= (self.z as u8) << 1;
        sr |= (self.i as u8) << 2;
        sr |= (self.d as u8) << 3;
        sr |= (self.b as u8) << 4;
        sr |= (true as u8) << 5; //expantion bit.
        sr |= (self.v as u8) << 6;
        sr |= (self.n as u8) << 7;
        sr
    }
    fn set_status_reg(&mut self, sr: u8) {
        self.n = (sr & 0b10000000) >> 7 != 0;
        self.v = (sr & 0b01000000) >> 6 != 0;
        //break flag is ignored
        self.d = (sr & 0b00001000) >> 3 != 0;
        self.i = (sr & 0b00000100) >> 2 != 0;
        self.z = (sr & 0b00000010) >> 1 != 0;
        self.c = (sr & 0b00000010) != 0;
    }
    fn push_ret(&mut self, cycles: &mut i32) {
        let part1 = ((self.PC + 1) >> 8) as u8;
        let part2 = ((self.PC + 2) as u8) & 0x00ff;
        self.stack_push(part1, cycles);
        self.stack_push(part2, cycles);
    }

    //general statuses for operations
    fn load_status(&mut self, reg: u8) {
        self.z = reg == 0;
        self.n = (reg >> 7) != 0;
    }
    fn adc_status(&mut self, reg: u8, carry: bool) {
        self.load_status(reg);
        self.c = carry;
        self.v = self.a > 127 || carry;
    }
    fn and_status(&mut self, reg: u8) {
        self.load_status(reg);
    }
    fn asl_status(&mut self, reg: u8, carry: bool) {
        self.load_status(reg);
        self.c = carry;
    }
    fn compare_status(&mut self, differnce: u16) {
        self.c = (differnce << 8) != 0;
        self.z = differnce == 0;
        self.n = (differnce << 7) != 0;
    }
    fn dec_inc_status(&mut self, reg: u8) {
        self.load_status(reg);
    }
    fn eor_status(&mut self) {
        //eor is only for accumulator
        self.load_status(self.a);
    }
    fn lsr_status(&mut self, carry: bool) {
        self.n = false;
        self.z = self.a == 0;
        self.c = carry;
    }
    fn ora_status(&mut self) {
        //ora is only for accumulator.
        self.load_status(self.a);
    }
    fn rotate_status(&mut self, carry: bool) {
        self.load_status(self.a);
        self.c = carry;
    }
    fn subtract_status(&mut self, carry: bool, overlow: bool) {
        self.c = carry;
        self.v = overlow;
        self.load_status(self.a);
    }
    fn transfer_status(&mut self, reg: u8) {
        self.load_status(reg);
    }

    //specific operation macros.
    fn lda_absolute(&mut self, loc: &mut u16, cycles: &mut i32) {
        self.absolute(loc, cycles);
        self.a = self.mem.read(*loc);
        self.load_status(self.a);
    }
    fn adc_set_a(&mut self, loc: u16) {
        //how the fuck does this take only one cycle!!! WTF
        let res = self.a as u16 + self.mem.read(loc) as u16;
        let c = res > 255;
        self.a = (res & 0x00ff) as u8;
        self.adc_status(self.a, c);
    }
    fn sbc_set_a(&mut self, loc: u16) {
        let res = self.a as i16 - self.mem.read(loc) as i16 - !self.c as i16;
        let c = res >= 0; 
        let v = res < 0; // !carrry
        self.a = (res & 0xff) as u8;
        self.subtract_status(c, v);
    }
    fn and_absolute(&mut self, reg: u8, loc: &mut u16, cycles: &mut i32) {
        self.absolute(loc, cycles);
        self.a &= self.mem.read(*loc);
        self.and_status(reg);
    }
    fn jump_absolute(&mut self, cycles: &mut i32) {
        let mut loc = self.mem.read(self.PC) as u16;
        self.PC += 1;
        loc |= (self.mem.read(self.PC) as u16) << 8;
        *cycles += 2;
        self.PC = loc;
    }
}
