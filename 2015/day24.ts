/*
  1. 3 groups with same weight
  2. Minimize packages on first group
  3. First group has QE, which is product of packages
  4. If there are multiple minimized combinations, take the one with the lower QE
*/

function computeSubsets(list: number[], sum: number) {
  function backtrack(curr: number, sum: number) {
    if (sum === 0) return [[]]

    let subsets: number[][] = []
    for (let i = curr+1; i < list.length; i++) {
      if (sum - list[i] < 0) continue
      const subset = backtrack(i, sum - list[i])
      for (const p of subset)
        subsets.push([list[i], ...p])
    }
  
    return subsets
  }

  return backtrack(-1, sum)
}

const GROUPS = 4

function solver(input: string) {
  let weights: number[] = []
  for(const line of input.split('\r\n'))
    weights.push(Number(line))

  const target = weights.reduce((acc, curr) => acc + curr, 0) / GROUPS
  let combinations = computeSubsets(weights, target)
  let smaller = Number.MAX_SAFE_INTEGER
  for(const comb of combinations) {
    smaller = Math.min(smaller, comb.length)
  }

  combinations = combinations.filter(comb => comb.length === smaller)
  let minQE = Number.MAX_SAFE_INTEGER
  for(const comb of combinations) {
    minQE = Math.min(minQE, comb.reduce((acc, curr) => acc * curr, 1))
  }

  return minQE
}

import execute from './handler'
execute(solver, 'day24.txt')