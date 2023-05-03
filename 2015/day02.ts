// https://adventofcode.com/2015/day/2

function solver1(input: string): number {
  let toOrder = 0

  for(const line of input.split('\n')) {
    const [l, w, h] = line.split('x').map(c => Number(c))
    const areas: number[] = [
      l*w, w*h, h*l
    ]
    const surface = 2*areas[0] + 2*areas[1] + 2*areas[2]
    const minArea = Math.min(...areas) 
    toOrder += surface + minArea
  }

  return toOrder
}

function solver2(input: string): number {
  let toOrder = 0

  for(const line of input.split('\n')) {
    const dimensions = line.split('x').map(c => Number(c))
    const shortestDist = dimensions.reduce((acc, curr) => acc + 2*curr, 0) - 2*Math.max(...dimensions)
    const volume = dimensions.reduce((acc, curr) => acc * curr, 1)
    toOrder += shortestDist + volume
  }

  return toOrder
}

import execute from './handler'
execute(solver2, 'day02.txt')