import fs from 'fs';
import { ReadDirectives } from './directives';
import PrettyPrint from './PrettyPrint';

export function ParseFile(path : string) {
    const data = fs.readFileSync(path, 'utf8');

    const directives = ReadDirectives(data);
    PrettyPrint(...directives);
    return directives;
}