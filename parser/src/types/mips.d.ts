export type GloblValue = string;
export type Instruction = string;
export type InstructionSet = Array<Instruction>;

export type DataDirectiveType = "word" | "asciiz";
export type BlockDirectiveType = "data" | "text";
export type MiscDirectiveType = "globl";

// Directives allowed to be children
export type ChildDirectiveType = DataDirectiveType;

export type DirectiveType = DataDirectiveType | BlockDirectiveType | MiscDirectiveType;

export interface BaseDirective<T extends DirectiveType> {
    type: T;
    args: string;
}

export interface GloblDirective extends BaseDirective<"globl"> {
    value: GloblValue;
}

// pun: asciiz "Bla"
export interface NamedDirective<T extends DataDirectiveType, V> extends BaseDirective<T> {
    value: V;
    name: string;
}

export type WordDirective = NamedDirective<"word", integer>;
export type AsciizDirective = NamedDirective<"asciiz", string>;

export interface BaseParentalDirective<
    T extends DirectiveType, 
    C extends ChildDirective
> extends BaseDirective<T> {
    children: Array<C>
}

export interface DataDirective extends BaseParentalDirective<"data", ChildDirective> {
    value: string; // Data dec
}

export interface TextDirective extends BaseDirective<"text"> {
    instructions: InstructionSet; // Code
}

export type Directive = GloblDirective | 
                        WordDirective | 
                        AsciizDirective |
                        DataDirective | 
                        TextDirective;

export type ParentalDirective = DataDirective;
export type ChildDirective = WordDirective | AsciizDirective;