import { ParseFile } from "./parser";
import { proceduresToC } from "./to_c";
import { TextDirective } from "./types/instructions";

import fs from "fs";

const parser = ParseFile("../mips_files/gaussian_sum.mips");

let text = "";

for(const d of parser) {
    if(d.type == "text") {
        text += proceduresToC((d as TextDirective).procedures);
    }
}

fs.writeFileSync("../mips_files/gaussian_sum.c", text);

// console.log(parser);