export type DirectiveParser = (label : string, args : string, text : string) => Directive;
export type Handlers = {
    [key : string]: DirectiveParser
}

export interface Directive {
    type : string;
    args : any[];
    label : string;
    children : Array<Directive>
}

