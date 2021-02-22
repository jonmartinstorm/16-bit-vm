const createMemory = require("./memory");
const CPU = require("./cpu");
const instructions = require("./instructions");

const memory = createMemory(256);
const writeableBytes = new Uint8Array(memory.buffer);

const cpu = new CPU(memory);

let i = 0;

writeableBytes[i++] = instructions.MOV_LIT_R1;
writeableBytes[i++] = 0x12;
writeableBytes[i++] = 0x34;

writeableBytes[i++] = instructions.MOV_LIT_R2;
writeableBytes[i++] = 0xAB;
writeableBytes[i++] = 0xCD;

writeableBytes[i++] = instructions.ADD_REG_REG;
writeableBytes[i++] = 0x02; // r1
writeableBytes[i++] = 0x03; // r2

cpu.debug();

cpu.step();
cpu.debug();

cpu.step();
cpu.debug();

cpu.step();
cpu.debug();