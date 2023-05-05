function findDivisorsSum(n: number): number {
  let sum = 0
  for(let i = 1; i <= Math.sqrt(n); ++i) {
    if (n % i == 0) {
      sum += i * 10
      sum += (n / i) !== i ? (n/i) * 10 : 0
    }
  }

  return sum
}

function findDivisorsSumAngry(n: number): number {
  let sum = 0
  let divisorsCount = 0
  for(let i = 1; i <= Math.sqrt(n); ++i) {
    if (n % i == 0) {
      sum += i * 11
      divisorsCount += 1
      if (divisorsCount === 50) break
      if (i !== n/i) {
        sum += (n/i) * 11
        divisorsCount += 1
      }
      if (divisorsCount === 50) break
    }
  }

  return sum
}

function solver(input: string, findDivisorsSum: (n: number) => number) {
  const target = Number(input)
  
  for(let i=1; i<Number.MAX_SAFE_INTEGER; ++i) {
    const sum = findDivisorsSum(i)
    if (sum >= target) return i
  }

  return -1
}

import execute from './handler'
execute((input) => solver(input, findDivisorsSumAngry), 'day20.txt')