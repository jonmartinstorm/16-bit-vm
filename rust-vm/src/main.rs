mod cpu;
mod instructions;
mod memory;

use std::io;

use std::iter;





use crate::memory::Memory;
use crate::cpu::CPU;

fn main() {
    let mut m = Memory::new(256*256);

    // Infinte iterator that increases 1 every time it is called
    let mut curr = 0;
    let mut i = iter::repeat_with(|| { let tmp = curr; curr += 1; tmp });

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

    m.set_uint8(i.next().unwrap(), instructions::MOV_MEM_REG);
    m.set_uint8(i.next().unwrap(), 0x01);
    m.set_uint8(i.next().unwrap(), 0x00);
    m.set_uint8(i.next().unwrap(), r1);

    m.set_uint8(i.next().unwrap(), instructions::MOV_LIT_REG);
    m.set_uint8(i.next().unwrap(), 0x00);
    m.set_uint8(i.next().unwrap(), 0x01);
    m.set_uint8(i.next().unwrap(), r2);

    m.set_uint8(i.next().unwrap(), instructions::ADD_REG_REG);
    m.set_uint8(i.next().unwrap(), r1);
    m.set_uint8(i.next().unwrap(), r2);

    m.set_uint8(i.next().unwrap(), instructions::MOV_REG_MEM); 
    m.set_uint8(i.next().unwrap(), acc);
    m.set_uint8(i.next().unwrap(), 0x01);
    m.set_uint8(i.next().unwrap(), 0x00);

    m.set_uint8(i.next().unwrap(), instructions::JMP_NOT_EQ);
    m.set_uint8(i.next().unwrap(), 0x00);
    m.set_uint8(i.next().unwrap(), 0x03);
    m.set_uint8(i.next().unwrap(), 0x00); // 0x0000 start:
    m.set_uint8(i.next().unwrap(), 0x00);
  
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
