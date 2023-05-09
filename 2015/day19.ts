function parseLine(line: string) {
  const tokens = line.split(' ')
  const src = tokens[0]
  const dst = tokens[2]

  return {src, dst}
}

type Graph = Map<string, string[]>

function parseData(input: string): [Graph, string] {
  let graph: Graph = new Map()
  
  const lines = input.split('\r\n')
  const startingMolecule = lines.pop()!

  // get rid of blank line
  lines.pop()

  for(const line of lines) {
    const edge = parseLine(line)
    
    const adj = graph.get(edge.src)
    if (adj) graph.set(edge.src, adj.concat(edge.dst))
    else graph.set(edge.src, [edge.dst])
  }

  return [graph, startingMolecule]
}

function getElements(molecule: string) {
  let elements: string[]= []
  for (let i = 0; i < molecule.length; i++) {
    let element = ''
    // if this happens it means its a 2-letters element
    if (i < molecule.length - 1 && 
      molecule[i+1].toLowerCase() === molecule[i+1]
    ) {
      element = molecule[i] + molecule[i+1]
      i += 1
    }
    else element = molecule[i]

    elements.push(element)
  }

  return elements
}

function solver1(input: string) {
  const [graph, startingMolecule] = parseData(input)
  const startingElements = getElements(startingMolecule)

  let createdMolecules = new Set<string>()
  for (let i = 0; i < startingElements.length; i++) {
    const replacements = graph.get(startingElements[i])
    if (!replacements) continue
    for(const replacement of replacements) {
      const newMolecule = [...startingElements]
      newMolecule[i] = replacement
      createdMolecules.add(newMolecule.join(''))
    }
  }

  return createdMolecules.size
}

function stepsFromStartToEnd(start: string[], end: string[], rules: Graph) {
  function rec (start: string[]) {
    if (start.join('') === end.join('')) return 0
    if (start.length > end.length) return Number.MAX_SAFE_INTEGER

    let minSteps = Number.MAX_SAFE_INTEGER

    for(let i = 0; i < start.length; i++) {
      const replacements = rules.get(start[i])
      if (!replacements) continue
      for (const replacement of replacements) {
        const elements = getElements(replacement)
        const newMolecule = [...start]

        newMolecule[i] = elements[0]
        for (let e = 1; e < elements.length; e++) {
          newMolecule.splice(i+e, 0, elements[e])
        }

        const steps = 1 + rec(newMolecule)
        minSteps = Math.min(steps, minSteps)
      }
    }
    
    return minSteps
  }

  return rec(start)
}

function solver2(input: string) {
  const [graph, endingMolecule] = parseData(input)
  let startingElements = ['e']
  let endingElements = getElements(endingMolecule)

  console.log(graph)

  const startTime = new Date()
  // too slow, works for simple input
  const res = stepsFromStartToEnd(startingElements, endingElements, graph)
  const endTime = new Date()

  console.log('Start:', startTime, 'End:', endTime)

  return res
}

import execute from './handler'
execute(solver2, 'day19.txt')