interface Point { x: number, y: number }

class Grid {
  grid: boolean[][]
  width: number
  height: number

  constructor (width: number, height: number) {
    this.grid = new Array(height)
    this.width = width
    this.height = height

    for(let i=0; i<height; ++i) {
      this.grid[i] = new Array(width).fill(false)
    }
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

  get(p: Point): boolean {
    return this.grid[p.y][p.x]
  }


  set(p: Point, value: boolean) {
    this.grid[p.y % this.height][p.x % this.width] = value
  }

  copy(other: Grid) {
    for(const [y, row] of other.grid.entries()) {
      for(const [x, state] of row.entries()) {
        const p = {x, y}
        this.set(p, other.get(p))
      }
    }
  }

  rect(width: number, height: number) {
    for(let y=0; y<height; ++y) {
      for(let x=0; x<width; ++x) {
        this.set({x, y}, true)
      }
    }
  }

  rotateRow(row: number, offset: number) {
    let temp = new Grid(this.width, this.height)
    
    for(let x=0; x<this.width; ++x) {
      const val = this.get({x, y: row})
      temp.set({x: x + offset, y: row}, val)
    }

    for(let x=0; x<this.width; ++x) {
      const val = temp.get({x, y: row})
      this.set({x, y: row}, val)
    }
  }
  
  rotateCol(col: number, offset: number) {
    let temp = new Grid(this.width, this.height)
    
    for(let y=0; y<this.height; ++y) {
      const val = this.get({x: col, y})
      temp.set({x: col, y: y + offset}, val)
    }

    for(let y=0; y<this.height; ++y) {
      const val = temp.get({x: col, y})
      this.set({x: col, y}, val)
    }
  }
}

interface RectI {
  op: 'rect'
  w: number
  h: number
}

interface RotateI {
  op: 'rotate'
  direction: string
  axis: number
  offset: number
}

type Instruction = RectI | RotateI

function parseLine(line: string): Instruction {
  const tokens = line.split(' ')
  const op = tokens[0]

  if(op === 'rect') {
    const dimensions = tokens[1].split('x')
    const w = Number(dimensions[0])
    const h = Number(dimensions[1])

    return {op: 'rect', w, h}
  } else {
    const direction = tokens[1]
    const axis = Number(tokens[2].split('=')[1])
    const offset = Number(tokens[4])

    return {op: 'rotate', direction, axis, offset}
  }
}

const W = 50
const H = 6

function solver(input: string) {
  let grid = new Grid(W, H)

  for(const line of input.split('\r\n')) {
    const instruction = parseLine(line)
    switch(instruction.op) {
      case 'rect':
        grid.rect(instruction.w, instruction.h)
        break
      case 'rotate':
        if (instruction.direction === 'row') {
          grid.rotateRow(instruction.axis, instruction.offset)
        } else {
          grid.rotateCol(instruction.axis, instruction.offset)
        }
        break
    }
  }

  grid.print()

  let count = 0
  for(let y=0; y<grid.height; ++y) {
    for(let x=0; x<grid.width; ++x) {
      count += grid.get({x, y}) ? 1 : 0
    }
  }

  return count
}

import execute from './handler'
execute(solver, 'day08.txt')