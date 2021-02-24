const createMemory = require("./memory");
const instructions = require("./instructions");

class CPU {
    constructor(memory) {
        this.memory = memory;

        this.registerNames = [
            "ip", "acc",
            "r1", "r2", "r3", "r4",
            "r5", "r6", "r7", "r8",
        ];

        // 16 bit registers, 2 bytes
        this.registers = createMemory(this.registerNames.length * 2);

        this.registerMap = this.registerNames.reduce((map, name, i) => {
            map[name] = i * 2;
            return map;
        }, {});
    }

    debug() {
        this.registerNames.forEach(name => {
            console.log(`${name}: 0x${this.getRegister(name).toString(16).padStart(4, "0")}`)
        });
        console.log();
    }

    viewMemoryAt(address) {
        const nextEightBytes = Array.from({length: 8}, (_, i) => 
            this.memory.getUint8(address + i)
        ).map(v => `0x${v.toString(16).padStart(2, "0")}`);

        console.log(`${address.toString(16).padStart(4, "0")}: ${nextEightBytes.join(" ")}`);
    }

    getRegister(name) {
        if (!(name in this.registerMap)) {
            throw new Error(`getRegister: no such register '${name}'`);
        }
        return this.registers.getUint16(this.registerMap[name]);
    }

    setRegister(name, value) {
        if (!(name in this.registerMap)) {
            throw new Error(`setRegister: no such register '${name}'`);
        }
        return this.registers.setUint16(this.registerMap[name], value);
    }

    fetch() {
        const nextInstructonAddress = this.getRegister("ip");
        const instruction = this.memory.getUint8(nextInstructonAddress);
        this.setRegister('ip', nextInstructonAddress + 1);
        return instruction;
    }

    fetch16() {
        const nextInstructonAddress = this.getRegister("ip");
        const instruction = this.memory.getUint16(nextInstructonAddress);
        this.setRegister('ip', nextInstructonAddress + 2);
        return instruction;
    }

    execute(instruction) {
        switch (instruction) {
            case instructions.MOV_LIT_REG: {
                const literal = this.fetch16();
                const register = (this.fetch() % this.registerNames.length) * 2;
                this.registers.setUint16(register, literal);
                return;
            }

            case instructions.MOV_REG_REG: {
                const registerFrom = (this.fetch() % this.registerNames.length) * 2;
                const registerTo = (this.fetch() % this.registerNames.length) * 2;
                const value = this.registers.getUint16(registerFrom);
                this.registers.setUint16(registerTo, value);
                return;
            }

            case instructions.MOV_REG_MEM: {
                const registerFrom = (this.fetch() % this.registerNames.length) * 2;
                const address = this.fetch16();
                const value = this.registers.getUint16(registerFrom);
                this.memory.setUint16(address, value);
                return;
            }

            case instructions.MOV_MEM_REG: {
                const address = this.fetch16();
                const registerTo = (this.fetch() % this.registerNames.length) * 2;
                const value = this.memory.getUint16(address);
                this.registers.setUint16(registerTo, value);
                return;
            }

            case instructions.ADD_REG_REG: {
                const r1 = this.fetch();
                const r2 = this.fetch();
                const registerValue1 = this.registers.getUint16(r1 * 2);
                const registerValue2 = this.registers.getUint16(r2 * 2);
                this.setRegister("acc", registerValue1 + registerValue2);
                return;
            }

            case instructions.JMP_NOT_EQ: {
                const literal = this.fetch16();
                const address = this.fetch16();

                if (literal !== this.getRegister("acc")) {
                    this.setRegister("ip", address);
                }

                return;
            }
        }
    }

    step() {
        const instruction = this.fetch();
        return this.execute(instruction);
    }
}

module.exports = CPU;