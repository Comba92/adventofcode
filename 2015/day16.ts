interface Aunt {
  [key: string]: number
}

function parseLine(line: string): any {
  const tokens = line.split(' ')
  let aunt: Aunt = {}
  for (let i=2; i<tokens.length-1; i+=2) {
    const key = tokens[i].replace(':', '')
    const value = Number( tokens[i+1].replace(',', '') )
    aunt[key] = value
  }

  return aunt
}

const target: Aunt = {
  children: 3,
  cats: 7,
  samoyeds: 2,
  pomeranians: 3,
  akitas: 0,
  vizslas: 0,
  goldfish: 5,
  trees: 3,
  cars: 2,
  perfumes: 1,
}

function isSubsetOf(smaller: Aunt, bigger: Aunt) {
  for (const [key, value] of Object.entries(smaller)) {
    if (bigger[key] !== value) return false
  }

  return true
}

function isSubSetOfWithRanges(smaller: Aunt, bigger: Aunt) {
  for (const [key, value] of Object.entries(smaller)) {
    
    switch(key) {
      case 'cats':
      case 'trees':
        if (value <= bigger[key]) {
          //console.log(key, value, 'isnt greater than', bigger[key])
          return false
        }
        break
      
      case 'pomeranians': 
      case 'goldfish':
        if (value >= bigger[key]) {
          //console.log(key, value,'isnt less than', bigger[key])
          return false
        }
        break

      default: 
        if (value !== bigger[key]) return false
        break
    }
  }

  return true
}

function solver(input: string, check: (a: Aunt, b: Aunt) => boolean) {
  const aunts = input.split('\n').map(line => parseLine(line))

  for (let i=0; i<aunts.length; ++i) {
    if (check(aunts[i], target)) return i+1
  }

  return -1
}

import execute from './handler'
execute((input) => solver(input, isSubSetOfWithRanges), 'day16.txt')