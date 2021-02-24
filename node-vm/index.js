const readline = require("readline");
const createMemory = require("./memory");
const CPU = require("./cpu");
const instructions = require("./instructions");

const IP  = 0;
const ACC = 1;
const R1  = 2;
const R2  = 3;

const memory = createMemory(256*256);
const writeableBytes = new Uint8Array(memory.buffer);

const cpu = new CPU(memory);

let i = 0;

// start:
//   mov #0x0100, r1
//   mov  0x0001, r2
//   add r1, r2
//   mov acc, #0x0100
//   jne 0x0003, start:

writeableBytes[i++] = instructions.MOV_MEM_REG;
writeableBytes[i++] = 0x01; // 0x0100
writeableBytes[i++] = 0x00;
writeableBytes[i++] = R1;

writeableBytes[i++] = instructions.MOV_LIT_REG;
writeableBytes[i++] = 0x00;
writeableBytes[i++] = 0x01;
writeableBytes[i++] = R2;

writeableBytes[i++] = instructions.ADD_REG_REG;
writeableBytes[i++] = R1;
writeableBytes[i++] = R2; 

writeableBytes[i++] = instructions.MOV_REG_MEM;
writeableBytes[i++] = ACC;
writeableBytes[i++] = 0x01; // 0x0100
writeableBytes[i++] = 0x00;

writeableBytes[i++] = instructions.JMP_NOT_EQ;
writeableBytes[i++] = 0x00;
writeableBytes[i++] = 0x03;
writeableBytes[i++] = 0x00; // 0x0000 start:
writeableBytes[i++] = 0x00;

cpu.step();
cpu.debug();
cpu.viewMemoryAt(cpu.getRegister("ip"));
cpu.viewMemoryAt(0x0100);

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
});

rl.on("line", () => {
    cpu.step();
    cpu.debug();
    cpu.viewMemoryAt(cpu.getRegister("ip"));
    cpu.viewMemoryAt(0x0100);
});