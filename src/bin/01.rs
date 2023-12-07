advent::solution!(1);

/*
OG Javascript part 1 implmation

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
*/

fn parse_input(input: &str, replace: bool) -> u32 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            if replace {
                line.to_string()
                    .replace("one", "one1one")
                    .replace("two", "two2two")
                    .replace("three", "three3three")
                    .replace("four", "four4four")
                    .replace("five", "five5five")
                    .replace("six", "six6six")
                    .replace("seven", "seven7seven")
                    .replace("eight", "eight8eight")
                    .replace("nine", "nine9nine")
            } else {
                line.to_string()
            }
        })
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
        .sum()
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    let output = parse_input(input, false) as usize;
    Some(output)
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    let output = parse_input(input, true) as usize;

    Some(output)
}
