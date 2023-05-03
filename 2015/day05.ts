// https://adventofcode.com/2015/day/5

function isNiceString1(s: string): boolean {
  const vowels = ['a', 'e', 'i', 'o', 'u']
  const excluded = ['ab', 'cd', 'pq', 'xy']
  let vowelsCount = 0;
  let oneRepeatedTwice = false

  for(let i=0; i<s.length-1; ++i) {
    if (vowels.includes(s[i])) vowelsCount += 1
    if (s[i] === s[i+1]) oneRepeatedTwice = true
    if (excluded.includes(s[i] + s[i+1])) return false
  }

  return oneRepeatedTwice && vowelsCount >= 3
}

function isNiceString2(s: string): boolean {
  let pairs = new Map<string, number>()
  let onePairWithNoOverlapping = false
  let lastPair = ''
  
  for(let i=0; i<s.length-1; ++i) {
    const pair = s[i] + s[i+1]
    if (pair !== lastPair) {
      const repetitions = pairs.get(pair)
      if (repetitions !== undefined) {
        pairs.set(pair, repetitions + 1)
        if (repetitions + 1 >= 2) onePairWithNoOverlapping = true
      } else pairs.set(pair, 0)
    }
    lastPair = pair
  }

  let palindromeOfLength3 = false
  for(let i=0; i<s.length-2; ++i) {
    if (s[i] === s[i+2])
      palindromeOfLength3 = true
  }

  return onePairWithNoOverlapping && palindromeOfLength3
}

function solver(input: string, isNiceString: (s: string) => boolean) {
  let niceStringsCount = 0
  for(const line of input.split('\n')) {
    niceStringsCount += isNiceString(line) ? 1 : 0
  }
  return niceStringsCount
}

import execute from './handler'
execute((input) => solver(input, isNiceString2), 'day05.txt')