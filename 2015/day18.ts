interface Point {
  x: number, y: number
}

const directions = [
  {x: 1, y: 1}, {x: -1, y: 1}, {x: 1, y: -1}, {x: -1, y: -1},
  {x: 1, y: 0}, {x: -1, y: 0}, {x: 0, y: 1}, {x: 0, y: -1},
]
const STEPS = 100

class Grid {
  grid: boolean[][]
  size: number

  constructor (size: number, init: boolean) {
    this.grid = new Array(size)
    this.size = size

    for(let i=0; i<this.grid.length; ++i) {
      this.grid[i] = new Array(size).fill(init)
    }
  }


  get(p: Point): boolean {
    return this.grid[p.y][p.x]
  }


  print() {
    console.log('')
    for(const [y, row] of this.grid.entries()) {
      let line = ''
      for(const [x, state] of row.entries()) {
        line += this.get({x, y}) ? '#' : '.'
      }
      console.log(line)
    }
    console.log('')
  }


  set(p: Point, value: boolean) {
    this.grid[p.y][p.x] = value
  }


  copy(other: Grid) {
    for(const [y, row] of other.grid.entries()) {
      for(const [x, state] of row.entries()) {
        const p = {x, y}
        this.set(p, other.get(p))
      }
    }
  }


  isInside(p: Point): boolean {
    return p.x >= 0 && p.x < this.size && p.y >= 0 && p.y < this.size
  }


  countNeighbors(p: Point): number {
    let count = 0
    for (const d of directions) {
      const np = {x: p.x+d.x, y: p.y+d.y}
      if (!this.isInside(np)) continue
      count += this.get(np) ? 1 : 0
    }
    return count
  }


  step(keepOn: Point[]) {
    let temp = new Grid(this.size, false)
    temp.copy(this)

    const keep = keepOn.map(p => p.x+','+p.y)

    for(const [y, row] of this.grid.entries()) {
      for(const [x, state] of row.entries()) {
        const p = {x, y}

        if(keep.includes(x+','+y)) {
          temp.set(p, true)
          continue
        }

        const neighbors = this.countNeighbors(p)
        if (state) {
          if (neighbors !== 2 && neighbors !== 3) temp.set(p, false)
        }
        else if (!state) {
          if (neighbors === 3) temp.set(p, true)
        }
        
      }
    }

    this.copy(temp)
  }
}

function solver(input: string) {
  let grid = new Grid(100, false)
  const keepOn = [
    {x:0, y:0}, {x:0, y:99}, {x:99, y:0}, {x:99, y:99}
  ]

  for(const [y, line] of input.split('\r\n').entries()) {
    for(const [x, state] of line.split('').entries()) {
        grid.set({x, y}, state === '#')
    }
  }
  for(const k of keepOn) grid.set(k, true)

  for(let i=0; i<STEPS; ++i) {
    grid.step(keepOn)
  }

  let count = 0
  for(const row of grid.grid) 
    for(const state of row) 
      if(state) count += 1

  return count
}

import execute from './handler'
execute(solver, 'day18.txt')