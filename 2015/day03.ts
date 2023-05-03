// https://adventofcode.com/2015/day/3

function solver1(input: string): number {
  let visited = new Set()
  let current = [0, 0]

  visited.add(current.toString())

  for(const direction of input.split('')) {
    switch(direction) {
      case '^': current[1] -= 1; break
      case 'v': current[1] += 1; break
      case '<': current[0] -= 1; break
      case '>': current[0] += 1; break
    }

    visited.add(current.toString())
  }

  return visited.size
}

function solver2(input: string): number {
  let visited = new Set()
  let pointers = [[0, 0], [0, 0]]
  let turn = 0

  visited.add([0,0].toString())

  for(const direction of input.split('')) {
    const current = pointers[turn]
    switch(direction) {
      case '^': current[1] -= 1; break
      case 'v': current[1] += 1; break
      case '<': current[0] -= 1; break
      case '>': current[0] += 1; break
    }

    visited.add(current.toString())
    turn = (turn+1) % pointers.length
  }

  return visited.size
}

import execute from './handler'
execute(solver2, 'day03.txt')