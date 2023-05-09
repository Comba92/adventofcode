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
  for(let i=0; i<s.length-1; ++i) {
    const pair = s[i] + s[i+1]

    if (s.substring(i+2).includes(pair)) return true
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
  for(const line of input.split('\r\n')) {
    niceStringsCount += isNiceString(line) ? 1 : 0
  }
  return niceStringsCount
}

import execute from './handler'
execute((input) => solver(input, isNiceString2), 'day05.txt')