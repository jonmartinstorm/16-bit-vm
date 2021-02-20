const createMemory = require("./memory");

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

    getRegister(name) {
        if (!(name in this.registerNames)) {
            throw new Error(`getRegister: no such register '${name}'`);
        }
        return this.registers.getUint16(this.registerMap[name]);
    }

    setRegister(name, value) {
        if (!(name in this.registerNames)) {
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

    execute(instruction) {
        switch (instruction) {

        }
    }
}

module.exports = CPU;