function increment(s: string): string {
  if (s.length === 0) return ''
  if (s.at(s.length-1) === 'z') return increment(s.slice(0, -1)) + 'a'
  return s.slice(0, -1) + String.fromCharCode( s.charCodeAt(s.length-1) + 1 )
}

function hasThreeIncreasingStraightChars(s: string): boolean {
  for(let i=0; i<s.length-2; ++i) {
    if (s.charCodeAt(i) + 1 === s.charCodeAt(i+1) &&
        s.charCodeAt(i+1) + 1 === s.charCodeAt(i+2) ) {
      return true
    }
  }

  return false
}

function hasNoExcludedChars(s: string, excluded: string[]): boolean {
  for(let i=0; i<s.length; ++i) {
    if (excluded.includes(s[i])) {
      return false
    }
  }

  return true
}

function countRepeatedNonOverlappingPairs(s: string): number {
  let pairs = new Set<string>()
  let lastPair = ''
  let count = 0
  for(let i=0; i<s.length-1; ++i) {
    // if it's a pair
    if (s[i] === s[i+1]) {
      const pair = s[i] + s[i+1]

      // control i haven't seen it already
      if (!pairs.has(pair)) {
        // if i haven't, add it to the set, increase count
        pairs.add(pair)
        count += 1
      }

      // if it's overlapping with the precedent, decrease count
      if (pair === lastPair) {
        count -= 1
      // or else i have to update the last pair seen 
      } else {
        lastPair = pair
      }
    }
  }

  return count
}
 
function solver(input: string) {
  const excluded = ['i', 'o', 'l']
  
  do { input = increment(input) }
  while (
    !hasThreeIncreasingStraightChars(input) ||
    !hasNoExcludedChars(input, excluded) ||
    countRepeatedNonOverlappingPairs(input) < 2
  )

  do { input = increment(input) }
  while (
    !hasThreeIncreasingStraightChars(input) ||
    !hasNoExcludedChars(input, excluded) ||
    countRepeatedNonOverlappingPairs(input) < 2
  )

  return input
}

import execute from './handler'
execute(solver, 'day11.txt')