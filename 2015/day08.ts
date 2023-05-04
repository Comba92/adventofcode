function countCharsOfData(s: string): number {
  let charsCount = 0
  for(let i=1; i<s.length-1; ++i) {
    if (s[i] == '\\') {
      i += 1
      if(s[i] === 'x') i += 2
    }

    charsCount += 1
  }

  return charsCount
}

function charIsDigit(c: string): boolean {
  return !Number.isNaN(Number(c))
}

function solver(input: string) {
  let sum = 0
  
  for (const line of input.split('\r\n')) {
    sum += line.length - countCharsOfData(line)
  }

  return sum
}

import execute from './handler'
execute(solver, 'day08.txt')