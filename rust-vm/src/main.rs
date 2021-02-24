mod cpu;
mod instructions;
mod memory;

use crate::memory::Memory;
use crate::cpu::CPU;

fn main() {
    let mut m = Memory::new(256*256);
    let mut i = 0;

    let IP  = 0;
    let ACC = 1;
    let R1  = 2;
    let R2  = 3;

    m.set_uint8(i, instructions::MOV_LIT_REG);
    i += 1;
    m.set_uint8(i, 0x12);
    i += 1;
    m.set_uint8(i, 0x34);
    i += 1;
    m.set_uint8(i, R1);
    i += 1;

    m.set_uint8(i, instructions::MOV_LIT_REG);
    i += 1;
    m.set_uint8(i, 0xAB);
    i += 1;
    m.set_uint8(i, 0xCD);
    i += 1;
    m.set_uint8(i, R2);
    i += 1;

    m.set_uint8(i, instructions::ADD_REG_REG);
    i += 1;
    m.set_uint8(i, 0x02);
    i += 1;
    m.set_uint8(i, 0x03);
    i += 1;

    m.set_uint8(i, instructions::MOV_REG_MEM);
    i += 1;
    m.set_uint8(i, ACC);
    i += 1;
    m.set_uint8(i, 0x01);
    i += 1;
    m.set_uint8(i, 0x00);
  
    let mut cpu = CPU::new(m);

    cpu.debug();
    cpu.view_memory_at(IP);
    cpu.view_memory_at(0x0100);

    cpu.step();
    cpu.debug();
    cpu.view_memory_at(IP);
    cpu.view_memory_at(0x0100);

    cpu.step();
    cpu.debug();
    cpu.view_memory_at(IP);
    cpu.view_memory_at(0x0100);

    cpu.step();
    cpu.debug();
    cpu.view_memory_at(IP);
    cpu.view_memory_at(0x0100);

    cpu.step();
    cpu.debug();
    cpu.view_memory_at(IP);
    cpu.view_memory_at(0x0100);

}
