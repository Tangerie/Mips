import { parseProcedures } from './instructions';
import { DirectiveParser, Handlers } from './types/directives';

const DIRECTIVE_REGEX = /(?:([a-zA-Z_]*)(?::*)) *\.([a-zA-Z_]+)(?: ?)([^\n\r]*)/g;

const handleAsciiz : DirectiveParser = (label : string, args : string, text : string) => {
    return {
        args: [args],
        label: label,
        type: "asciiz",
        children: []
    }
}

const handleWord : DirectiveParser = (label : string, args : string, text : string) => {
    return {
        args: [args],
        label: label,
        type: "word",
        children: []
    }
}

const handleGlobl : DirectiveParser = (label : string, args : string, text : string) => {
    return {
        args: [args],
        label: label,
        type: "globl",
        children: []
    }
}

const handleText : DirectiveParser = (label : string, args : string, text : string) => {
    const procedures = parseProcedures(text);

    return {
        args: [args],
        label: label,
        type: "text",
        children: [],
        procedures: procedures
    }
}

const DataHandlers : Handlers = {
    "asciiz": handleAsciiz,
    "word": handleWord,
}

const handleData : DirectiveParser = (label : string, args : string, text : string) => {
    // Parse types
    
    const children = ReadDirectives(text, DataHandlers);

    return {
        args: [args],
        label: label,
        type: "data",
        children: children
    }
}

// name : [end_previous_block, handler]
export const DirectiveHandlers : Handlers = {
    "globl": handleGlobl,
    "text": handleText,
    "data": handleData,
}

export function ReadDirectives(text : string, handlers = DirectiveHandlers) {
    const matches = [...text.matchAll(DIRECTIVE_REGEX)];

    const directives = [];

    for(let i = 0; i < matches.length; i++) {
        const match = matches[i];
        
        const start = match.index ?? 0;

        let [raw, label, directive, args] = match;

        if(!handlers[directive]) {
            continue;
        }

        const handler = handlers[directive];


        let j = i + 1;
        let next = matches.at(j);
        while(next && !handlers[next.at(2) ?? ""]) {
            j++;
            next = matches.at(j);
        }
        let end = next?.index;

        if (!end || end < 0) {
            end = text.length;
        }

        const dir = handler(label, args, text.substring(start, end));
        
        directives.push(dir); 
    }

    return directives;
}