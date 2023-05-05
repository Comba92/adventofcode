function isCharNumber(c: string): boolean {
  if (c === '-') return true
  return !Number.isNaN(Number(c))
}

function isCharLetter(c: string): boolean {
  return c.toUpperCase() !== c.toLowerCase()
}

function getNumber(s: string, i: number) {
  let number = 0
  let negative = false
  do {
    if (s[i] === '-')
      negative = true
    else 
      number = number * 10 + Number(s[i])
    i += 1
  } while(isCharNumber(s[i]))

  return negative ? -number : number
}

function getWord(s: string, i: number) {
  let word = ''

  do {
    word += s[i]
    i += 1
  } while(isCharLetter(s[i]))

  return word
}

function reachEndOfObject(s: string, i: number) {
  let depth = 1

  while (depth > 0) {
    i += 1
    if (s[i] == '{') depth += 1
    else if (s[i] == '}') depth -= 1
  }

  return i
}

function solver1(input: string) {
  let sum = 0

  let i = 0
  while(i < input.length) {
    if (isCharNumber(input[i])) {
      const number = getNumber(input, i)
      i = i + number.toString().length
      sum += number
    } else i += 1
  }

  return sum
}

enum Struct {
  Array, Object
}

function solver2(input: string) {
  let stack: {struct: Struct, sum: number}[] = []
  let sum = 0

  let i = 0
  while(i < input.length) {
    if (input[i] == '[') stack.push({struct: Struct.Array, sum: 0})
    else if (input[i] == '{') stack.push({struct: Struct.Object, sum: 0})
    else if (input[i] == ']' || input[i] == '}') {
      const last = stack.pop()
      if(last) {
        if (stack.length === 0) sum += last.sum
        else stack[stack.length-1].sum += last.sum
      } 
    }

    else if (isCharNumber(input[i])) {
      const number = getNumber(input, i)
      i = i + number.toString().length - 1
      stack[stack.length-1].sum += number
    }

    else if (isCharLetter(input[i])) {
      const word = getWord(input, i)
      i = i + word.length - 1
      if (word === 'red' && stack[stack.length-1].struct === Struct.Object) {
        i = reachEndOfObject(input, i)
        stack.pop()
      }
    }

    i += 1
  }

  return sum
}


import execute from './handler'
execute(solver2, 'day12.txt')