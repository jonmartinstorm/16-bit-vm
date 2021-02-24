mod cpu;
mod instructions;
mod memory;

use std::io;

use crate::memory::Memory;
use crate::cpu::CPU;

fn main() {
    let mut m = Memory::new(256*256);
    let mut i = 0;

    let ip  = 0;
    let acc = 1;
    let r1  = 2;
    let r2  = 3;

    // start:
    //   mov #0x0100, r1
    //   mov  0x0001, r2
    //   add r1, r2
    //   mov acc, #0x0100
    //   jne 0x0003, start:

    m.set_uint8(i, instructions::MOV_MEM_REG);
    i += 1;
    m.set_uint8(i, 0x01);
    i += 1;
    m.set_uint8(i, 0x00);
    i += 1;
    m.set_uint8(i, r1);
    i += 1;

    m.set_uint8(i, instructions::MOV_LIT_REG);
    i += 1;
    m.set_uint8(i, 0x00);
    i += 1;
    m.set_uint8(i, 0x01);
    i += 1;
    m.set_uint8(i, r2);
    i += 1;

    m.set_uint8(i, instructions::ADD_REG_REG);
    i += 1;
    m.set_uint8(i, r1);
    i += 1;
    m.set_uint8(i, r2);
    i += 1;

    m.set_uint8(i, instructions::MOV_REG_MEM);
    i += 1;
    m.set_uint8(i, acc);
    i += 1;
    m.set_uint8(i, 0x01);
    i += 1;
    m.set_uint8(i, 0x00);
    i += 1;

    m.set_uint8(i, instructions::JMP_NOT_EQ);
    i += 1;
    m.set_uint8(i, 0x00);
    i += 1;
    m.set_uint8(i, 0x03);
    i += 1;
    m.set_uint8(i, 0x00); // 0x0000 start:
    i += 1;
    m.set_uint8(i, 0x00);
  
    let mut cpu = CPU::new(m);

    cpu.debug();
    cpu.view_memory_at(cpu.get_register("ip".to_string()));
    cpu.view_memory_at(0x0100);

    let mut rl = String::new();
    loop {
        io::stdin().read_line(&mut rl).unwrap();

        cpu.step();
        cpu.debug();
        cpu.view_memory_at(cpu.get_register("ip".to_string()));
        cpu.view_memory_at(0x0100);
    }
}
