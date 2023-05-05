interface Deer {
  speed: number,
  timeOn: number,
  timeOff: number,
  cycle: number,
  distancePerCycle: number
}

function parseLine(line: string): Deer {
  const tokens = line.split(' ')
  const speed = Number(tokens.at(3))
  const timeOn = Number(tokens.at(6))
  const timeOff = Number(tokens.at(-2))
  const cycle = timeOn + timeOff
  const distancePerCycle = timeOn * speed

  return {speed, timeOn, timeOff, cycle, distancePerCycle}
}

function computeDistance(deer: Deer, seconds: number) {
  if (seconds % deer.cycle <= deer.timeOn) {
    return Math.floor(seconds / deer.cycle) * deer.distancePerCycle + (seconds % deer.cycle) * deer.speed
  } else {
    return Math.floor(seconds / deer.cycle) * deer.distancePerCycle + deer.distancePerCycle
  }
}

const END_TIME = 2503

function solver1(input: string) {
  let winner = 0

  for (const line of input.split('\n')) {
    const deer = parseLine(line) 
    let distance = computeDistance(deer, END_TIME)
    winner = Math.max(distance, winner)
  }

  return winner
}

function solver2(input: string) {
  let deers: Deer[] = []
  
  for (const line of input.split('\n')) {
    deers.push(parseLine(line)) 
  }

  let distances: number[] = Array(deers.length).fill(0)
  let scores: number[] = Array(deers.length).fill(0)

  for(let sec=1; sec<=END_TIME; ++sec) {
    let maxDistance = 0
    let distance = 0
    for (let i=0; i<deers.length; ++i) {
      distances[i] = computeDistance(deers[i], sec)
      maxDistance = Math.max(distances[i], maxDistance)
    }

    for (let i=0; i<deers.length; ++i) {
      if (distances[i] === maxDistance) {
        scores[i] += 1
      }
    }
  }

  return Math.max(...scores)
}

import execute from './handler'
execute(solver2, 'day14.txt')