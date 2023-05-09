function getNext(code: number) {
  return (code * 252533) % 33554393 
}

const LAST_ROW = 2978
const LAST_COL = 3083
const STARTING_VALUE = 20151125

function solver(input: string) {
  let curr = STARTING_VALUE

  let x = 1, y = 2
  let lastRowReached = 2
  while (x !== LAST_COL || y !== LAST_ROW) {
    curr = getNext(curr)
    if (y === 1) {
      lastRowReached += 1
      y = lastRowReached
      x = 1
    } else {
      x += 1
      y -=1
    }
  }

  return getNext(curr)
}

import execute from './handler'
execute(solver, 'day25.txt')