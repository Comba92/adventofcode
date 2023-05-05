// https://adventofcode.com/2015/day/5

function isNiceString1(s: string): boolean {
  const vowels = ['a', 'e', 'i', 'o', 'u']
  const excluded = ['ab', 'cd', 'pq', 'xy']
  let vowelsCount = 0;
  let oneRepeatedTwice = false

  for(let i=0; i<s.length-1; ++i) {
    // count vowels
    if (vowels.includes(s[i])) vowelsCount += 1
    // one letter repeated twice in a row
    if (s[i] === s[i+1]) oneRepeatedTwice = true
    // does not contain exlucded letters
    if (excluded.includes(s[i] + s[i+1])) return false
  }

  return oneRepeatedTwice && vowelsCount >= 3
}

function hasPairOf2RepeatedNonOverlappingChars(s: string): boolean {
  let pairs = new Set<string>()
  let lastPair = ''
  for(let i=0; i<s.length-1; ++i) {
    const pair = s[i] + s[i+1]

    // check if i already seen it
    if (pairs.has(pair)) {
      // if i have seen it and they are the same
      if (s[i] === s[i+1]) {
        // if they don't overlap, they are a correct pair
        if (s[i+1] !== s[i+2]) return true
      // if i have seen it and they aren't the same,  they are a correct pair
      } else return true

    // if i haven't seen it, add it to the set and update the last seen
    } else {
      pairs.add(pair)
      lastPair = pair
    }
  }

  return false
}

function hasPalindromeOfLength3(s: string): boolean {
  for(let i=0; i<s.length-2; ++i) {
    if (s[i] === s[i+2])
      return true
  }

  return false
}

function isNiceString2(s: string): boolean {
  console.log(hasPairOf2RepeatedNonOverlappingChars(s), hasPalindromeOfLength3(s))
  
  return hasPairOf2RepeatedNonOverlappingChars(s) && 
    hasPalindromeOfLength3(s)
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