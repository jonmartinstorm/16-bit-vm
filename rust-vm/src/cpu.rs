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
        let registers = Memory::new(register_names.len() * 2);
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
            println!("{}: {:#06x}", name, self.get_register(name.to_string()));
        }
        println!("");
    }

    pub fn view_memory_at(&self, address: u16) {
        let next_eight_bytes: Vec<String> = vec!["".to_string(); 8].into_iter().enumerate()
            .map(|(i, _v)| {
                format!("{:#04x}", self.memory.get_uint8(address as usize + i))
        }).collect();

        println!("{:#06x}: {}", address, next_eight_bytes.join(" "));
    }

    pub fn get_register(&self, name: String) -> u16{
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
            instructions::MOV_LIT_REG => {
                let literal = self.fetch16();
                let register = (self.fetch() as usize % self.register_names.len()) * 2;
                self.registers.set_uint16(register, literal);
            },

            instructions::MOV_REG_REG => {
                let register_from = (self.fetch() as usize % self.register_names.len()) * 2;
                let register_to = (self.fetch() as usize % self.register_names.len()) * 2;
                let value = self.registers.get_uint16(register_from.into());
                self.registers.set_uint16(register_to, value);
            },

            instructions::MOV_REG_MEM => {
                let register_from = (self.fetch() as usize % self.register_names.len()) * 2;
                let address = self.fetch16();
                let value = self.registers.get_uint16((register_from).into());
                self.memory.set_uint16(address.into(), value);
            },

            instructions::MOV_MEM_REG => {
                let address = self.fetch16();
                let register_to = (self.fetch() as usize % self.register_names.len()) * 2;
                let value = self.memory.get_uint16(address.into());
                self.registers.set_uint16(register_to, value);
            },

            instructions::ADD_REG_REG => {
                let r1 = self.fetch();
                let r2 = self.fetch();
                let register_value1 = self.registers.get_uint16((r1 * 2).into());
                let register_value2 = self.registers.get_uint16((r2 * 2).into());
                self.set_register("acc".to_string(), register_value1 + register_value2);
            },

            instructions::JMP_NOT_EQ => {
                let literal = self.fetch16();
                let address = self.fetch16();

                if literal != self.get_register("acc".to_string()) {
                    self.set_register("ip".to_string(), address);
                }
            },

            _ => {},
        }
    }

    pub fn step(&mut self) {
        let instruction = self.fetch();
        self.execute(instruction);
    }

}