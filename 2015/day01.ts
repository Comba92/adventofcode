// https://adventofcode.com/2015/day/1

function solver1(input: string) {
  return input
    .split('')
    .reduce((acc, curr) => acc + (curr === '(' ? 1 : -1), 0)
}

function solver2(input: string) {
  let floor = 0
  for(let i=0; i<input.length; ++i) {
    if (input[i] === '(') floor += 1
    else if (input[i] == ')') floor -= 1

    if (floor === -1) return i+1
  }
}

import execute from "./handler"

execute(solver2, 'day01.txt')