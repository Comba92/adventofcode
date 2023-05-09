function getValue(source: string, wires: Map<string, number>): number {
  const value = Number(source)
  if (Number.isNaN(value)) {
    const signal =  wires.get(source)

    if (signal === undefined) throw new Error('No signal!')
    
    return signal
  } else return value
}

function applyBitwise(op1: number, op2: number | undefined, op: string) {
  let result = 0

  if (op2 === undefined) result = new Uint16Array([~op1])[0]
  else switch(op) {
    case    'AND': result = new Uint16Array([op1 & op2])[0]; break
    case     'OR': result = new Uint16Array([op1 | op2])[0]; break
    case 'LSHIFT': result = new Uint16Array([op1 << op2])[0]; break
    case 'RSHIFT': result = new Uint16Array([op1 >> op2])[0]; break
  }

  return result
}

function emulateLine(line: string, wires: Map<string, number>) {
  const tokens = line.split(' ')

  if(tokens.length === 3) {

    const signal = getValue(tokens[0], wires)
    const destination = tokens[2]
    wires.set(destination, signal)

  } else if (tokens.length === 4) {

    const signal = getValue(tokens[1], wires)
    const destination = tokens[3]
    const result = applyBitwise(signal, undefined, 'NOT')
    wires.set(destination, result)

  } else {

    const op1 = getValue(tokens[0], wires)
    const op2 = getValue(tokens[2], wires)
    const op = tokens[1]
    const destination = tokens[4]
    const result = applyBitwise(op1, op2, op)
    wires.set(destination, result)
  }
}

function solver(input: string) {
  let wires = new Map<string, number>()
  let instructionsLeft: Set<string> = new Set()

  for (const line of input.split('\r\n')) {
    instructionsLeft.add(line)
  }
 
  while(instructionsLeft.size > 0) {
    for(const line of instructionsLeft) {
      try {
        emulateLine(line, wires)
        instructionsLeft.delete(line)
      } catch(e) { }
    }
  }

  const aSignal = wires.get('a')
  wires.clear()
  if (aSignal !== undefined) wires.set('b', aSignal)
  
  for (const line of input.split('\r\n')) {
    instructionsLeft.add(line)
  }
  instructionsLeft.add(aSignal + ' -> b')
 
  while(instructionsLeft.size > 0) {
    for(const line of instructionsLeft) {
      try {
        emulateLine(line, wires)
        instructionsLeft.delete(line)
      } catch(e) { continue }
    }
  }

  return wires.get('a')
}

function solverOptimized(input: string) {
  let wires = new Map<string, number>()
  let instructionsLeft: Set<string> = new Set()
  let graph = new Map<string, string[]>()

  for (const line of input.split('\r\n')) {
    instructionsLeft.add(line)
  }
 
  while(instructionsLeft.size > 0) {
    for(const line of instructionsLeft) {
      try {
        emulateLine(line, wires)
        instructionsLeft.delete(line)
      } catch(e) { }
    }
  }

  const aSignal = wires.get('a')
  wires.clear()
  if (aSignal !== undefined) wires.set('b', aSignal)
  
  for (const line of input.split('\r\n')) {
    instructionsLeft.add(line)
  }
  instructionsLeft.add(aSignal + ' -> b')
 
  while(instructionsLeft.size > 0) {
    for(const line of instructionsLeft) {
      try {
        emulateLine(line, wires)
        instructionsLeft.delete(line)
      } catch(e) { continue }
    }
  }

  return wires.get('a')
}

import execute from './handler'
execute(solver, 'day07.txt')