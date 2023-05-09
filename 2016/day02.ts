enum Direction {
  Right = 'R',
  Down = 'D',
  Left = 'L',
  Up = 'U'
}

function solver1(input: string) {
  let keys = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
  ]
  let position = [1, 1]
  let code = 0

  const instructions = input.split('\r\n')
  for(const instruction of instructions) {
    for(const direction of instruction) {
      let move: [number, number] = [0, 0]
      switch(direction) {
        case Direction.Up:    move = [0, -1]; break
        case Direction.Down:  move = [0, 1]; break
        case Direction.Left:  move = [-1, 0]; break
        case Direction.Right: move = [1, 0]; break
      }

      let newposition = [position[0] + move[0], position[1] + move[1]]
      if (
        newposition[0] < 0 || newposition[0] >= keys[0].length ||
        newposition[1] < 0 || newposition[1] >= keys.length
      ) continue
      
      position = newposition
    }

    code = code*10 + keys[position[1]][position[0]]
  }

  return code
}

function solver2(input: string) {
  let keys = [
    [ '', '',  '1',  '',  ''],
    [ '', '2', '3', '4',  ''],
    ['5', '6', '7', '8', '9'],
    [ '', 'A', 'B', 'C',  ''],
    [ '',  '', 'D',  '',  '']
  ]
  let position = [0, 2]
  let code = ''

  const instructions = input.split('\r\n')
  for(const instruction of instructions) {
    for(const direction of instruction) {
      let move: [number, number] = [0, 0]
      switch(direction) {
        case Direction.Up:    move = [0, -1]; break
        case Direction.Down:  move = [0, 1]; break
        case Direction.Left:  move = [-1, 0]; break
        case Direction.Right: move = [1, 0]; break
      }

      let newposition = [position[0] + move[0], position[1] + move[1]]
      if (
        newposition[0] < 0 || newposition[0] >= keys[0].length ||
        newposition[1] < 0 || newposition[1] >= keys.length ||
        keys[newposition[1]][newposition[0]] === ''
      ) continue
      
      position = newposition
    }

    code = code + keys[position[1]][position[0]]
  }

  return code
}

import execute from './handler'
execute(solver2, 'day02.txt')