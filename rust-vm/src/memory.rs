
#[derive (Debug)]
pub struct Memory {
    array: Vec<u8>,
}

impl Memory {
    pub fn new(size_in_bytes: usize) -> Memory {
        Memory {
            array: vec![0; size_in_bytes],
        }
    }

    pub fn get_uint8(&self, index: usize) -> u8{
        //TODO: chech for index outside array.
        self.array[index]
    }

    pub fn get_uint16(&self, index: usize) -> u16{
        //TODO: chech for index outside array.
        let h = self.array[index];
        let l = self.array[index + 1];
        ((h as u16) << 8) + l as u16
    }

    pub fn set_uint8(&mut self, index: usize, value: u8) {
        self.array[index] = value;
    }

    pub fn set_uint16(&mut self, index: usize, value: u16) {
        self.array[index + 1] = (value & 0x00ff) as u8;
        self.array[index] = ((value & 0xff00) >> 8) as u8;
    }
}