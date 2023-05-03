// https://adventofcode.com/2015/day/6

enum Action {
  Toggle,
  On, 
  Off
}

interface Point {
  x: number, 
  y: number
}

interface Instruction {
  action: Action,
  p1: Point, p2: Point
}

class Grid<T> {
  grid: T[][]
  size: number
  rules: Map<Action, (old: T) => T>

  constructor (size: number, init: T, rules: Map<Action, (old: T) => T>) {
    this.grid = new Array(size)
    this.size = size
    this.rules = rules

    for(let i=0; i<this.grid.length; ++i) {
      this.grid[i] = new Array(size).fill(init)
    }
  }

  get(p: Point): T {
    return this.grid[p.y][p.x]
  }

  getLike(value: T): number {
    let count = 0
    for(let y=0; y<this.size; ++y) {
      for(let x=0; x<this.size; ++x) {
        count += this.get({x, y}) === value ? 1 : 0
      }
    }

    return count
  }

  getSum(): number {
    let sum = 0
    for(let y=0; y<this.size; ++y) {
      for(let x=0; x<this.size; ++x) {
        sum += Number(this.get({x, y}))
      }
    }

    return sum
  }

  set(p: Point, value: T) {
    this.grid[p.y][p.x] = value
  }


  do(p: Point, a: Action) {
    const setter = this.rules.get(a)
    if (setter === undefined) return

    const oldValue = this.get(p)
    const newValue = setter(oldValue)
    this.set(p, newValue) 
  }

  rect(i: Instruction) {
    const sx = Math.min(i.p1.x, i.p2.x)
    const sy = Math.min(i.p1.y, i.p2.y)
    const dx = Math.max(i.p1.x, i.p2.x)
    const dy = Math.max(i.p1.y, i.p2.y)

    for(let y=sy; y<=dy; ++y) {
      for(let x=sx; x<=dx; ++x) {
        this.do({x, y}, i.action)
      }
    }
  }
}

function parseLine(line: string): Instruction {
  let p1: Point, p2: Point, action: Action
  const tokens = line.split(' ')

  // its either turn off or turn on
  if (tokens.length === 5) {
    action = tokens[1] == 'on' ? Action.On : Action.Off
    const [x1, y1] = tokens[2].split(',')
    const [x2, y2] = tokens[4].split(',')
    p1 = {x: Number(x1), y: Number(y1)}
    p2 = {x: Number(x2), y: Number(y2)}
  } else {
    action = Action.Toggle
    const [x1, y1] = tokens[1].split(',')
    const [x2, y2] = tokens[3].split(',')
    p1 = {x: Number(x1), y: Number(y1)}
    p2 = {x: Number(x2), y: Number(y2)}
  }

  return { p1, p2, action }
}

function solver1(input: string): number {
  let rules = new Map([
    [Action.On, () => true],
    [Action.Off, () => false],
    [Action.Toggle, (old: boolean) => !old]
  ])
  let grid = new Grid<boolean>(1000, false, rules)

  for(const line of input.split('\n')) {
    const instruction = parseLine(line)
    grid.rect(instruction)
  }

  return grid.getLike(true)
}

function solver2(input: string): number {
  let rules = new Map([
    [Action.On, (old: number) => old+1], 
    [Action.Off, (old: number) => old > 0 ? old-1 : old],
    [Action.Toggle, (old: number) => old+2]
  ])
  let grid = new Grid<number>(1000, 0, rules)

  for(const line of input.split('\n')) {
    const instruction = parseLine(line)
    grid.rect(instruction)
  }

  return grid.getSum()
}

import execute from './handler'
execute(solver2, 'day06.txt')