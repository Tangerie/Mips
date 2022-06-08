import { InstructionMap, SyscallMap } from "./types/instructions";

export const Instructions : InstructionMap = {
    "li": (instruction, context) => {
        return [`${instruction.args[0]} = ${instruction.args[1]};`, {
            [instruction.args[0]]: parseInt(instruction.args[1])
        }];
    },
    "la": (instruction, context) => {
        return [`${instruction.args[0]} = ${instruction.args[1]};`, {
            [instruction.args[0]]: instruction.args[1]
        }];
    },
    "move": (instruction, context) => {
        return [`${instruction.args[0]} = ${instruction.args[1]};`, {
            [instruction.args[0]]: instruction.args[1]
        }];
    },
    "syscall": (instruct, context) => {
        const cmd = context["$v0"];
        if (cmd in Syscall) {
            return Syscall[cmd](context);
        }

        return ["syscall", {}];
    },
    "add": ({args: [a, b, c]}, context) => {
        return [
            `${a} = ${b} + ${c};`,
            {
                [a]: context[b] + context[c]
            }
        ]
    },
    "addi": ({args: [a, b, c]}, context) => {
        return [
            `${a} = ${b} + ${c};`,
            {
                [a]: context[b] + parseInt(c)
            }
        ]
    },
    "sub": ({args: [a, b, c]}, context) => {
        return [
            `${a} = ${b} - ${c};`,
            {
                [a]: context[b] - context[c]
            }
        ]
    },
    "subi": ({args: [a, b, c]}, context) => {
        return [
            `${a} = ${b} - ${c};`,
            {
                [a]: context[b] - parseInt(c)
            }
        ]
    },
    "mul": ({args: [a, b, c]}, context) => {
        return [
            `${a} = ${b} * ${c};`,
            {
                [a]: context[b] * context[c]
            }
        ]
    },
    "div": ({args: [a, b, c]}, context) => {
        return [
            `${a} = ${b} / ${c};`,
            {
                [a]: context[b] / context[c]
            }
        ]
    },
    "rem": ({args: [a, b, c]}, context) => {
        return [
            `${a} = ${b} % ${c}`,
            {
                [a]: context[b] % context[c]
            }
        ]
    },
    "abs": ({args: [a, b]}, context) => {
        return [
            `${a} = abs(b);`,
            {
                [a]: Math.abs(context[b])
            }
        ]
    },
    "neg": ({args: [a, b]}, context) => {
        return [
            `${a} = -b;`,
            {
                [a]: -context[b]
            }
        ]
    },
}

Instructions.addu = Instructions.add;
Instructions.addiu = Instructions.addi;
Instructions.subu = Instructions.sub;
Instructions.subiu = Instructions.subi;
Instructions.divu = Instructions.div;
Instructions.remu = Instructions.rem;

const Syscall : SyscallMap = {
    1: (context) => [`printf("%d", a0);`, {}],
    2: (context) => [`printf("%lf", f12);`, {}],
    3: (context) => [`printf("%lf", f12);`, {}],
    4: (context) => [`printf("%s", a0);`, {}],
    5: (context) => [`scanf("%d", &v0);`, {"$v0": 1}],
    6: (context) => [`scanf("%lf", &f0);`, {"$f0": 1}],
    7: (context) => [`scanf("%lf", &f0);`, {"$f0": 1}],
    8: (context) => [`fgets($a0, &a1, stdin);`, {"$a0": "Input String", "$a1": 1}],

    10: (context) => [`exit(0);`, {}],
    11: (context) => [`printf("%c", a0);`, {}],
    12: (context) => [`scanf("%c", &a0);`, {"$a0": 'c'}],
    13: (context) => [`exit(a0);`, {}],
}