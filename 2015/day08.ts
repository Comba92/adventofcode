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

function encode(s: string): string {
  let ns = ''
  for (const c of s) {
    switch(c) {
      case '"': ns += '\\"'; break
      case '\\': ns += '\\\\'; break
      default: ns += c
    }
  }

  return '"' + ns + '"'
}

function solver1(input: string) {
  let sum = 0
  
  for (const line of input.split('\r\n')) {
    sum += line.length - countCharsOfData(line)
  }

  return sum
}

function solver2(input: string) {
  let sum = 0
  
  for (const line of input.split('\r\n')) {
    sum += encode(line).length - line.length
  }

  return sum
}

import execute from './handler'
execute(solver2, 'day08.txt')