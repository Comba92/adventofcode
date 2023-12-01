interface Destination {
  type: string
  id: number
}

interface GiveInstruction {
  type: 'Give'
  id: number
  high: Destination
  low: Destination
}

interface AssignInstruction {
  type: 'Assign'
  id: number
  chip: number
}

type Instruction = GiveInstruction | AssignInstruction

function parseLine(line: string): Instruction {
  const tokens = line.split(' ')

  if (tokens.at(0) === 'value') {
    const id = Number(tokens.at(-1))
    const chip = Number(tokens.at(1))

    const i: Instruction = {type: 'Assign', id, chip}
    return i
  } else {
    const id = Number(tokens.at(1))
    
    const lowDstName = tokens.at(5)!
    const lowDstId = Number(tokens.at(6))
    const low: Destination = {type: lowDstName, id: lowDstId}

    const highDstName = tokens.at(-2)!
    const highDstId = Number(tokens.at(-1))
    const high: Destination = {type: highDstName, id: highDstId}

    const i: Instruction = {type: 'Give', id, low, high}
    return i
  }
}

function executeLine(line: string, entities: Map<string, number[]>) {
  const i = parseLine(line)
  if (i.type === 'Assign') {
    const bot = entities.get('bot'+i.id)
    if (bot) {
      entities.set('bot'+i.id, bot.concat(i.chip))
    } else {
      entities.set('bot'+i.id, [i.chip])
    }
  } else {
    const bot = entities.get('bot'+i.id)
    if (bot === undefined || bot.length < 2) throw new Error(`Bot ${i.id} hasn't got 2 chips yet!`)

    bot.sort((a, b) => a-b)
    const [chipLow, chipHigh] = bot

    const otherLow = entities.get(i.low.type+i.low.id)
    if (otherLow) {
      otherLow.push(chipLow)
    } else {
      entities.set(i.low.type+i.low.id, [chipLow])
    }

    const otherHigh = entities.get(i.high.type+i.high.id)
    if (otherHigh) {
      otherHigh.push(chipHigh)
    } else {
      entities.set(i.high.type+i.high.id, [chipHigh])
    }
  }
}

function solver(input: string, search: (entities: Map<string, number[]>) => number) {
  let instructionsLeft = new Set<string>()
  let entities = new Map<string, number[]>

  for (const line of input.split('\r\n')) {
    instructionsLeft.add(line)
  }
 
  while(instructionsLeft.size > 0) {
    for(const line of instructionsLeft) {
      try {
        executeLine(line, entities)
        instructionsLeft.delete(line)
      } catch(e) { }
    }
  }

  return search(entities)
}

function problem1(entities: Map<string, number[]>) {
  return Array.from(entities)
    .find( ([key, value]) => value.includes(61) && value.includes(17))
}

function problem2(entities: Map<string, number[]>) {
  const bins = [0, 1, 2]
  return Array.from(entities)
    .filter( ([key, value]) => key === 'output0' || key === 'output1' || key === 'output2')
    .map( ([key, value]) => value)
    .reduce((acc, curr) => acc * curr[0], 1)
}

import execute from './handler'
execute((input) => solver(input, problem2), 'day10.txt')