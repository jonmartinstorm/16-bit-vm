use std::collections::HashMap;

use crate::memory::Memory;
use crate::instructions;

#[derive (Debug)]
pub struct CPU {
    memory: Memory,
    register_names: Vec<String>,
    registers: Memory,
    register_map: HashMap<String, usize>,
}

impl CPU {
    pub fn new(memory: Memory) -> CPU {
        let register_names =  vec![
            "ip".to_string(), "acc".to_string(),
            "r1".to_string(), "r2".to_string(), "r3".to_string(), "r4".to_string(),
            "r5".to_string(), "r6".to_string(), "r7".to_string(), "r8".to_string(),
        ];
        let registers = Memory::new(register_names.len());
        let register_map =  register_names.iter().enumerate().fold(HashMap::new(), |mut map: HashMap<String, usize>, (i, name)| {
            map.insert(
                name.to_string(), i * 2,
            );
            map
        });

        CPU {
            memory,
            register_names,
            register_map,
            registers,
            
        }
    }

    pub fn debug(&self) {
        for name in &self.register_names {
            println!("{}: {:#x}", name, self.get_register(name.to_string()));
        }
        println!("");
    }

    fn get_register(&self, name: String) -> u16{
        // TODO: check for register in registermap
        self.registers.get_uint16(self.register_map[&name])
    }

    fn set_register(&mut self, name: String, value: u16) {
        // TODO: check for register in registermap
        self.registers.set_uint16(self.register_map[&name], value);
    }

    fn fetch(&mut self) -> u8{
        let next_instruction_address = self.get_register("ip".to_string());
        let instruction = self.memory.get_uint8(next_instruction_address.into());
        self.set_register("ip".to_string(), next_instruction_address + 1);
        return instruction;
    }

    fn fetch16(&mut self) -> u16{
        let next_instruction_address = self.get_register("ip".to_string());
        let instruction = self.memory.get_uint16(next_instruction_address.into());
        self.set_register("ip".to_string(), next_instruction_address + 2);
        return instruction;
    }

    fn execute(&mut self, instruction: u8) {
        match instruction {
            instructions::MOV_LIT_R1 => {
                let literal = self.fetch16();
                self.set_register("r1".to_string(), literal);
            },

            instructions::MOV_LIT_R2 => {
                let literal = self.fetch16();
                self.set_register("r2".to_string(), literal);
            },

            instructions::ADD_REG_REG => {
                let r1 = self.fetch();
                let r2 = self.fetch();
                let register_value1 = self.registers.get_uint16((r1 * 2).into());
                let register_value2 = self.registers.get_uint16((r2 * 2).into());
                self.set_register("acc".to_string(), register_value1 + register_value2);
            },

            _ => {},
        }
    }

    pub fn step(&mut self) {
        let instruction = self.fetch();
        self.execute(instruction);
    }

}