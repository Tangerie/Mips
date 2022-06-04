import './config';

import fs from 'fs';
import { parseText } from './parser';


const TEST_FILE = "./test/bad_pun.mips"

const data = fs.readFileSync(TEST_FILE, 'utf8');

parseText(data);