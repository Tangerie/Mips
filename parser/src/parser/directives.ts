import { ChildDirective, ChildDirectiveType, DataDirective, DataDirectiveType, Directive, DirectiveType, ParentalDirective, TextDirective } from "../types/mips";

const DIRECTIVE_REGEX = /(?:([a-zA-Z_]*)(?::*)) *\.([a-zA-Z_]+)(?: ?)([^\n\r]*)/g;

// Return an array of Directives
/*
Single Line:
    - .globl
    - .word     (.data)
    - .asciiz   (.data)

Multi Line:
    - .data
    - .text
*/
export function findDirectives(text : string) : Directive[] {
    const found : Directive[] = [];

    const matches = [...text.matchAll(DIRECTIVE_REGEX)];

    let parent : ParentalDirective | null = null;

    for(const [raw, name, dir, args, index, ..._] of matches) {
        const directiveType = dir as DirectiveType;
        let data : Directive | null;

        switch(directiveType) {
            case "globl":
                parent = null;
                data = {
                    type: directiveType,
                    args: "",
                    value: args
                }
                break;
            case "asciiz":
                data = {
                    type: directiveType,
                    args: "",
                    value: args,
                    name: name
                }
                break;
            case "word":
                data = {
                    type: directiveType,
                    args: "",
                    value: parseInt(args),
                    name: name
                }
                break;
            case "data": 
                data = {
                    type: directiveType,
                    args: args,
                    value: "",
                    children: []
                }
                parent = data;
                break;
            case "text":
                data = {
                    type: directiveType,
                    args: args,
                    instructions: []
                }
                parent = null;
                break;
            default:
                data = null;
        }

        if(data) {
            if(parent && parent != data) {
                parent.children.push(data as ChildDirective);
            } else {
                found.push(data);
            }
        }
    }

    return found;
}

/*
.text
all code

.data
all data (with namespaces)
*/
export function reconstruct(dirs : Directive[]) : string {
    let lines : string[] = [];

    const textDirectives : TextDirective[] = dirs.filter(d => d.type == "text") as TextDirective[];
    const dataDirectives : DataDirective[] = dirs.filter(d => d.type == "data") as DataDirective[];

    lines.push(".text");
    for(const text of textDirectives) {
        // Replace data names with namespaced
        lines.push(...text.instructions);
    }

    lines.push('.data');
    for(const data of dataDirectives) {
        for(const child of data.children) {
            lines.push(`    ${data.args == "" ? "" : data.args + '_'}${child.name}: ${child.type} ${child.value}`)
        }
    }

    return lines.join("\n");
}