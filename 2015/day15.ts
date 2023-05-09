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

function range(a: number, b: number) {
  let r: number[] = []
  for (let i=a; i<=b; i++) {
    r.push(i)
  }
  return r
}

const MAX_INGREDIENTS = 100

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