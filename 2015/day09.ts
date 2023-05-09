function parseLine(line: string) {
  const tokens = line.split(' ')
  const src = tokens[0]
  const dst = tokens[2]
  const weight = Number(tokens[4])

  return {src, dst, weight}
}

import { computePermutations } from './utils'

function solver(input: string) {
  let cities = new Set<string>()
  let edges = new Map<string, number>()

  for(const line of input.split('\r\n')) {
    const route = parseLine(line)

    cities.add(route.src)
    cities.add(route.dst)

    edges.set(route.src+route.dst, route.weight)
    edges.set(route.dst+route.src, route.weight)
  }

  let citiesPermutations = computePermutations(Array.from(cities))
  let minRoute = Number.MAX_SAFE_INTEGER
  let maxRoute = Number.MIN_SAFE_INTEGER

  for(const route of citiesPermutations) {
    let dist = 0
    for(let i=1; i<route.length; ++i) {
      dist += edges.get(route[i-1]+route[i])!
    }

    //minRoute = Math.min(minRoute, dist)
    maxRoute = Math.max(maxRoute, dist)
  }

  return maxRoute
}

import execute from './handler'
execute(solver, 'day09.txt')