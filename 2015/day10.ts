function turn(s: string): string {
  let ns = ''
  let currentDigit = s[0]
  let currentCount = 1
  for(let i=1; i<s.length; ++i) {
    if (currentDigit === s[i]) currentCount += 1
    else {
      ns += currentCount + currentDigit
      currentDigit = s[i]
      currentCount = 1
    }
  }
  ns += currentCount + currentDigit

  return ns
}

function solver(input: string) {
  for(let i=0; i<50; ++i) {
    input = turn(input)
  }

  return input.length
}

import execute from './handler'
execute(solver, 'day10.txt')