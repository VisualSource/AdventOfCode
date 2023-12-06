import { readFile } from "node:fs/promises";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";

import assert from 'node:assert';

const data = await readFile(resolve(dirname(fileURLToPath(import.meta.url)), "./input2.txt"), { encoding: "utf-8" });

let output: number = 0;

const lines: string[] = data.split("\n");

for (let line of lines) {
    line = line
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
    let item = "";
    for (const v of line.matchAll(/\d/g)) {
        item += v[0];
    }

    console.log(line, item[0] + item[item.length - 1]);


    if (item.length) output += parseInt(item[0] + item[item.length - 1]);
}


console.log(output);

//assert(output === 142);


function test(input: string) {
    const lines = input.split("\n").filter(line => !!line.length);

    lines.map(line => {
        return line.replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine")
    }).map(line=>{

        




    });






}