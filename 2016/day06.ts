function increasingSort(a: [string, number], b: [string, number]) { return b[1] - a[1] }
function decreasingSort(a: [string, number], b: [string, number]) { return a[1] - b[1] }

type SortFunction = (a: [string, number], b: [string, number]) => number

function solver(input: string, sort: SortFunction) {
  const lines = input.split('\r\n')
  let occurences: Array<Map<string, number>> = new Array(lines[0].length) 
  
  for (let i = 0; i < occurences.length; i++) {
    occurences[i] = new Map<string, number>()
  }
  
  for(const line of lines) {
    for (let i = 0; i < line.length; i++) {
      const count = occurences[i].get(line[i])
      if (count) occurences[i].set(line[i], count+1)
      else occurences[i].set(line[i], 1)
    }
  }

  let message = ''
  for (let i = 0; i < occurences.length; i++) {
    const maxOccurence = Array
      .from(occurences[i])
      .sort( sort ) 

    // most common letter
    message += maxOccurence[0][0]
  }

  return message
}

import execute from './handler'
execute((input) => solver(input, decreasingSort), 'day06.txt')