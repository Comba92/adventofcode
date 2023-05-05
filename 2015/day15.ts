interface Ingredient {
  properties: number[],
  calories: number
}

function parseLine(line: string): Ingredient {
  const tokens = line.split(' ')
  let properties: string[] = []
  properties.push(tokens[2])
  properties.push(tokens[4])
  properties.push(tokens[6])
  properties.push(tokens[8])

  return {
    properties: properties.map(p => Number(p)),
    calories: Number(tokens[10])
  }
}

function solver(input: string) {
  let maxScore = 0

  const descriptions = input.split('\n')
  let ingredients: Ingredient[] = []

  for (const line of descriptions) {
    const ingredient = parseLine(line)
    ingredients.push(ingredient)
  }

  return maxScore
}

import execute from './handler'
execute(solver, 'day15.txt')