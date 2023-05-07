function parseLine(line: string) {
  const tokens = line.split(' ')
  const src = tokens[0]
  const dst = tokens[10].replace('.', '')
  const sign = tokens[2] === 'gain' ? 1 : -1
  const weight = sign*Number(tokens[3])

  return {src, dst, weight}
}

// https://stackoverflow.com/questions/52356091/recursive-algorithm-to-generate-all-permutations-of-length-k-of-a-list-in-python
function computeSubsets<T>(list: T[], size: number) {
  function backtrack(curr: number, size: number) {
    if (size <= 0) return [[]]

    let subsets: T[][] = []
    for (let i = curr+1; i < list.length; i++) {
      const subset = backtrack(i, size-1)
      for (const p of subset)
        subsets.push([list[i], ...p])
    }
  
    return subsets
  }

  return backtrack(-1, size)
}

function computePermutations<T>(list: T[]) { 
  let perms: T[][] = []

  function backtrack(sublist: T[]) {
    if (sublist.length === list.length) {
      perms.push(sublist)
      return 
    }

    for (let i = 0; i < list.length; i++) {
      if (sublist.includes(list[i])) continue
      sublist.push(list[i])
      backtrack([...sublist])
      sublist.pop()
    }
  }

  backtrack([])
  return perms
}

function solver(input: string) {
  let people = new Set<string>()
  let graph = new Map<string, number>()
  
  for(const line of input.split('\r\n')) {
    const route = parseLine(line)
    people.add(route.src)
    people.add(route.dst)

/*     const edge = [route.src, route.dst].sort().join('')
    const weight = graph.get(edge)
    if (weight) graph.set(edge, weight + route.weight)
    else graph.set(edge, route.weight) */

    const hash1 = route.src+route.dst
    const weight1 = graph.get(hash1)
    if (weight1) graph.set(hash1, weight1 + route.weight)
    else graph.set(hash1, route.weight)
    
    const hash2 = route.dst+route.src
    const weight2 = graph.get(hash2)
    if (weight2) graph.set(hash2, weight2 + route.weight)
    else graph.set(hash2, route.weight) 

/*     const edge = graph.get(route.src)
    if (edge) graph.set(route.src, edge.concat([[route.dst, route.weight]]))
    else graph.set(route.src, [[route.dst, route.weight]]) */
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


  // TOO LOW!
  return maxHappiness
}

import execute from './handler'
execute(solver, 'day13.txt')