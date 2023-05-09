interface Ingredient {
  name: string
  properties: number[]
  calories: number
}

function parseLine(line: string): Ingredient {
  const tokens = line.split(' ')
  let properties: string[] = []
  properties.push(tokens[2].replace(',', ''))
  properties.push(tokens[4].replace(',', ''))
  properties.push(tokens[6].replace(',', ''))
  properties.push(tokens[8].replace(',', ''))

  return {
    name: tokens[0].replace(':', ''),
    properties: properties.map(p => Number(p)),
    calories: Number(tokens[10])
  }
}

function getIngredients(input: string) {
  const descriptions = input.split('\n')
  let ingredients: Ingredient[] = []

  for (const line of descriptions) {
    const ingredient = parseLine(line)
    ingredients.push(ingredient)
  }

  return ingredients
}


const MAX_INGREDIENTS = 100
import { computeSubsets, computePermutations, range } from './utils'

function solver1(input: string) {
  const ingredients = getIngredients(input)
  
  const combinations = computeSubsets(range(0, MAX_INGREDIENTS), ingredients.length)
    .filter(comb => comb.reduce((acc, curr) => acc + curr, 0) === MAX_INGREDIENTS)

  let maxScore = Number.MIN_SAFE_INTEGER
  for(const comb of combinations) {
    const perms = computePermutations(comb)
    for(const perm of perms) {
      let totalScore = 1
      for(let p=0; p<ingredients[0].properties.length; ++p) {
        let propertyScore = 0
        for(let i=0; i<ingredients.length; ++i) {
          propertyScore += perm[i] * ingredients[i].properties[p]
        }
        if (propertyScore < 0) propertyScore = 0
        totalScore *= propertyScore
      }

      maxScore = Math.max(totalScore, maxScore)
    }
  }

  return maxScore
}

const MAX_CALORIES = 500

function solver2(input: string) {
  const ingredients = getIngredients(input)
  
  const combinations = computeSubsets(range(0, MAX_INGREDIENTS), ingredients.length)
    .filter(comb => comb.reduce((acc, curr) => acc + curr, 0) === MAX_INGREDIENTS)

  let maxScore = Number.MIN_SAFE_INTEGER
  for(const comb of combinations) {
    const perms = computePermutations(comb)
    for(const perm of perms) {
      let calories = 0
      for(let i=0; i<ingredients.length; ++i) {
        calories += perm[i] * ingredients[i].calories
      }
      if (calories !== MAX_CALORIES) continue

      let totalScore = 1
      for(let p=0; p<ingredients[0].properties.length; ++p) {
        let propertyScore = 0
        for(let i=0; i<ingredients.length; ++i) {
          propertyScore += perm[i] * ingredients[i].properties[p]
        }
        if (propertyScore < 0) propertyScore = 0
        totalScore *= propertyScore
      }

      maxScore = Math.max(totalScore, maxScore)
    }
  }

  return maxScore
}

import execute from './handler'
execute(solver2, 'day15.txt')