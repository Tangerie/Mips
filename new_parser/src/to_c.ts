import { Instructions } from "./MipsC";
import { Instruction, InstructionContext, Procedures } from "./types/instructions";

export function proceduresToC(procedures : Procedures) : string {
    let lines : string[] = [];

    let context : InstructionContext = {};

    for (const p of procedures) {
        lines.push(`void ${p.name}() {`);

        for(const key in context) {
            lines.push("// " + key + " = " + context[key]);
        }

        const [new_lines, new_context] = instructionsToC(p.instructions, context);

        context = Object.assign(context, new_context);

        lines = [...lines, ...new_lines.map(x => "\t" + x)];

        lines.push('}\n');

        lines.push("\n// Ending Context");
        for(const key in context) {
            lines.push("// " + key + " = " + context[key]);
        }

        
    }

    return lines.join("\n");
}

const JS_TO_C_TYPES = {
    string: "char *",
    number: "int ",
    bigint: "int ",
    boolean: "int ",
    symbol: "unknown ",
    object: "void ",
    undefined: "void ",
    function: "void "
}

function instructionsToC(instructions : Instruction[], context : InstructionContext = {}) : [string[], InstructionContext] {
    let lines : string[] = [];

    for(const i of instructions) {
        if(i.command == "") {
            lines.push("// " + i.comment);
            continue;
        }
        if(Instructions[i.command]) {
            i.args = i.args.map(x => x.replace("$", ""));
            let [l, ctxt] = Instructions[i.command](i, context);
            if(i.comment) {
                l += " // " + i.comment;
            }
            lines.push(l);
            context = Object.assign(context, ctxt);
        } else {
            lines.push(`${i.command} ${i.args.join(", ")} ${i.comment ? "# " + i.comment : ""}`)
        }
    }

    const context_lines : string[] = [];

    for(const [key, value] of Object.entries(context)) {
        const var_type = JS_TO_C_TYPES[typeof value];
        
        context_lines.push(`${var_type}${key};`)
    }

    lines = [
        ...context_lines,
        ...lines
    ]

    return [lines, context];
}