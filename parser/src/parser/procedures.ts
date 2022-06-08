import { Directive, Instruction, Procedure, TextDirective } from "../types/mips";

const PROCEDURE_NAME_REGEX = /([a-zA-Z_]*):/g

export function parseInstructions(text : string, directives : Directive[]) : Procedure[] {
    const procedures = getProcedureData(text, directives);

    return procedures;
}

// Gets all the procedure data and positions
// Doesnt collect instructions
export function getProcedureData(text : string, directives : Directive[]) : Procedure[] {
    const procedures : Procedure[] = [];

    const textDirectives = directives.filter(d => d.type == "text") as TextDirective[];

    let previous_prod : Procedure | null = null;

    let i;
    for(i = 0; i < textDirectives.length; i++) {
        const dir = textDirectives[i];
        const nextDirective = directives.at(directives.indexOf(dir) + 1);

        const target_text = text.substring(dir.start + dir.type.length + 1, nextDirective?.start);

        const lines = target_text.split("\n");
        for(const line of lines) {

            const match = line.match(PROCEDURE_NAME_REGEX);

            if(match) {
                const name = match[1];
                const prod = {
                    instructions: [],
                    name: name,
                    origin: dir,
                    start: match.index as number
                }
                procedures.push(prod);
                previous_prod = prod;
            } else if(previous_prod) {
                previous_prod.instructions.push(parseInstructionLine(line));
            }
        }
    }

    return procedures;
}

export function parseInstructionLine(line : string) : Instruction {
    line = line.trim();
    return line;
}