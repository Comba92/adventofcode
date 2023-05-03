// https://adventofcode.com/2015/day/4

import md5 from 'md5'

function solver(input: string, zeroes: number): number {
  for(let i=0; i<Number.MAX_SAFE_INTEGER; ++i) {
    const hash = md5(input + i.toString())
    if (i % (10 ** 6) == 0) console.log(i / (10 ** 6) + ' million hashes computed...')
    if(hash.slice(0, zeroes) === '0'.repeat(zeroes)) return i
  }

  return -1
}

import execute from './handler'
execute((input) => solver(input, 6), 'day04.txt')