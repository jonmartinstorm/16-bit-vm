mod cpu;
mod instructions;
mod memory;

use crate::memory::Memory;
use crate::cpu::CPU;

fn main() {
    let mut m = Memory::new(256);
    let mut i = 0;

    m.set_uint8(i, instructions::MOV_LIT_R1);
    i += 1;
    m.set_uint8(i, 0x12);
    i += 1;
    m.set_uint8(i, 0x34);
    i += 1;

    m.set_uint8(i, instructions::MOV_LIT_R2);
    i += 1;
    m.set_uint8(i, 0xAB);
    i += 1;
    m.set_uint8(i, 0xCD);
    i += 1;

    m.set_uint8(i, instructions::ADD_REG_REG);
    i += 1;
    m.set_uint8(i, 0x02);
    i += 1;
    m.set_uint8(i, 0x03);
  
    let mut cpu = CPU::new(m);

    cpu.debug();

    cpu.step();
    cpu.debug();

    cpu.step();
    cpu.debug();

    cpu.step();
    cpu.debug();

}
