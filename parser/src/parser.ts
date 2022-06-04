import { findDirectives, reconstruct } from "./parser/directives";
import PrettyPrint from "./PrettyPrint";

export function parseText(text : string) {
    const ds = findDirectives(text);

    console.log("\n=== DIRECTIVES ===");
    PrettyPrint(...ds);

    console.log(reconstruct(ds));
}