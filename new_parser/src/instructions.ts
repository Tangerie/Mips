import { Procedure, Procedures } from "./types/instructions";

const PROCEDURE_REGEX = /^[A-Za-z_-]+:/g
const COMMENT_REGEX = /#(.*)/g

export function parseProcedures(text : string) : Procedures {
    const procedures : Procedures = [];

    const lines = text.split('\n').map(x => x.trim()).slice(1);
    let currentProcedure : Procedure | null = null;

    for(let line of lines) {
        
        const commentIndex = line.search(COMMENT_REGEX);
        
        let comment = commentIndex >= 0 ? line.substring(commentIndex) : null;
        
        if(comment) {
            line = line.substring(0, commentIndex)
        }

        const match = line.match(PROCEDURE_REGEX);
        if (match) {
            currentProcedure = {
                name: line.split(":")[0],
                instructions: []
            }
            procedures.push(currentProcedure);
            continue;
        }

        let [command, ...args_sp] = line.split(" ");
        const arg_string = args_sp.join(" ");

        const args = arg_string.split(",").map(x => x.trim()).filter(x => x != '');

        if(currentProcedure) {
            if(comment || command != "") {
                currentProcedure.instructions.push({
                    command: command,
                    args: args,
                    comment: comment?.slice(1)?.trim()
                })
            }
        }
    }

    return procedures;
}