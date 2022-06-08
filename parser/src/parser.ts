import { findDirectives } from "./parser/directives";
import { parseInstructions } from "./parser/procedures";
import PrettyPrint from "./PrettyPrint";

export function parseText(text : string) {
    const ds = findDirectives(text);

    console.log("\n=== DIRECTIVES ===");
    PrettyPrint(...ds);

    console.log("\n=== PROCEDURES ===");
    const ps = parseInstructions(text, ds);
    PrettyPrint(...ps);
}