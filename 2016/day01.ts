enum Direction {
  Right = 0,
  Down = 1,
  Left = 2,
  Up = 3
}

function updatePosition(position: [number, number], direction: Direction, distance: number) {
  switch(direction) {
    case Direction.Up: position[1] -= distance; break
    case Direction.Down: position[1] += distance; break
    case Direction.Left: position[0] -= distance; break
    case Direction.Right: position[0] += distance; break
  }
}

function solver1(input: string) {
  let position: [number, number] = [0, 0]
  let direction = Direction.Up

  for(let instruction of input.split(', ')) {
    direction = direction + (instruction[0] === 'R' ? 1 : -1)
    if (direction < 0) direction = 3
    else if (direction > 3) direction = 0
    
    const distance = Number(instruction.slice(1))
    updatePosition(position, direction, distance)
  }
  return Math.abs(position[0]) + Math.abs(position[1])
}

function solver2(input: string) {
  let position: [number, number] = [0, 0]
  let direction = Direction.Up
  let visited = new Set<string>()

  for(let instruction of input.split(', ')) {
    direction = direction + (instruction[0] === 'R' ? 1 : -1)
    if (direction < 0) direction = 3
    else if (direction > 3) direction = 0
    
    const distance = Number(instruction.slice(1))
    for (let i = 0; i < distance; i++) {
      updatePosition(position, direction, 1)
      if (visited.has(position.toString())) {
        return Math.abs(position[0]) + Math.abs(position[1])
       } else visited.add(position.toString())
    }
  }
}

import execute from './handler'
execute(solver2, 'day01.txt')