function parseLine(line: string) {
  const tokens = line.split(' ')
  const src = tokens[0]
  const dst = tokens[10].replace('.', '')
  const sign = tokens[2] === 'gain' ? 1 : -1
  const weight = sign*Number(tokens[3])

  return {src, dst, weight}
}

import { computePermutations } from './utils'

function solver(input: string) {
  let people = new Set<string>()
  let graph = new Map<string, number>()
  
  for(const line of input.split('\r\n')) {
    const route = parseLine(line)
    people.add(route.src)
    people.add(route.dst)

    const hash1 = route.src+route.dst
    const weight1 = graph.get(hash1)
    if (weight1) graph.set(hash1, weight1 + route.weight)
    else graph.set(hash1, route.weight)
    
    const hash2 = route.dst+route.src
    const weight2 = graph.get(hash2)
    if (weight2) graph.set(hash2, weight2 + route.weight)
    else graph.set(hash2, route.weight) 
  }

  for(const person of people) {
    graph.set('You'+person, 0)
    graph.set(person+'You', 0)
  }
  people.add('You')

  console.log(graph)

  const permutations = computePermutations(Array.from(people))
  let maxHappiness = Number.MIN_SAFE_INTEGER
  for(const order of permutations) {
    let happines = 0
    for(let i=1; i<order.length; ++i) {
      happines += graph.get(order[i-1]+order[i])!
    }
    happines += graph.get(order[order.length-1]+order[0])!
    
    maxHappiness = Math.max(maxHappiness, happines) 
  }

  return maxHappiness
}

import execute from './handler'
execute(solver, 'day13.txt')