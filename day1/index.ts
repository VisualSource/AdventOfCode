import { readFile } from "node:fs/promises";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";

import assert from 'node:assert';

const data = await readFile(resolve(dirname(fileURLToPath(import.meta.url)), "./input.txt"), { encoding: "utf-8" });

const isNum = (value: string) => /\d/.test(value);

const values: number[] = [];

let line: string = "";

for (let i = 0; i < data.length; i++) {

    if (data[i] === "\n" || i === data.length - 1) {
        const number = line[0] + line[line.length - 1];
        values.push(parseInt(number));
        line = "";
        continue;
    }

    if (isNum(data[i])) {
        line += data[i];
    }
}


const output = values.reduce((pre, curr) => pre + curr, 0);
console.log(output)

//assert(output === 142);
