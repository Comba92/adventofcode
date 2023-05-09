function supportsTSL(line: string): boolean {
  let insideBrackets = false
  let hasABBA = false
  for (let i = 0; i < line.length-3; i++) {
    if(line[i] === '[') insideBrackets = true
    else if(line[i] === ']') insideBrackets = false

    else {
      if (
        line[i] === line[i+3] &&
        line[i+1] === line[i+2] &&
        line[i] !== line[i+1]
      ) {
        if (insideBrackets) {
          hasABBA = false
          break
        }
        hasABBA = true
      }
    }
  }

  return hasABBA
}

function supportsSSL(line: string): boolean {
  let BABs = new Set<string>()
  let insideBrackets = false

  // search for BABs, and make set of possible ABAs
  for (let i = 0; i < line.length-2; i++) {
    if(line[i] === '[') insideBrackets = true
    else if(line[i] === ']') insideBrackets = false

    else if(insideBrackets) {
      const seq = line.slice(i, i+3)
      if (seq[0] === seq[2] && seq[0] !== seq[1]) {
        const reversedSeq = seq[1]+seq[0]+seq[1]
        BABs.add(reversedSeq)
      }
    }
  }

  // search for ABAs, and check if have corresponding BABs
  insideBrackets = false
  for (let i = 0; i < line.length-2; i++) {
    if(line[i] === '[') insideBrackets = true
    else if(line[i] === ']') insideBrackets = false

    else if(!insideBrackets) {
      const seq = line.slice(i, i+3)
      if (seq[0] === seq[2] && seq[0] !== seq[1] && BABs.has(seq))
        return true
    }
  }

  return false
}

function solver(input: string, check: (s: string) => boolean) {
  let count = 0
  
  for(const line of input.split('\r\n')) {
    if (check(line)) count += 1
  }

  return count
}

import execute from './handler'
execute((input) => solver(input, supportsSSL), 'day07.txt')