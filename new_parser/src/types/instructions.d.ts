import { Directive } from "./directives";

export interface Instruction {
    command : string;
    args: string[];
    comment?: string;
}

export interface Procedure {
    name : string;
    instructions : Array<Instruction>;
}

export type Procedures = Array<Procedure>;

export interface TextDirective extends Directive {
    procedures: Procedures
}

export interface InstructionMap {
    [command : string] : (instruction : Instruction, context : InstructionContext) => [string, InstructionContext];
}

export interface SyscallMap {
    [key : number] : (context : InstructionContext) => [string, InstructionContext];
}

export interface InstructionContext {
    [register : string] : any;
}