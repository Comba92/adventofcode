function solver1(input: string) {
  let sum = 0
  
  for(const line of input.split('\r\n')) {
    for (let i = 0; i < line.length; i++) {
      if (line[i] === '(') {
        let marker = ''
        i += 1
        while(line[i] !== ')') {
          marker += line[i]
          i += 1
        }
        const [size, repetitions] = marker.split('x').map(v => Number(v))
        sum += size*repetitions
        i += size
      }

      else sum += 1
    }
  }

  return sum
}

function getMarker(s: string, position: number) {
  let marker = ''
  position += 1
  while(s[position] !== ')') {
    marker += s[position]
    position += 1
  }

  return marker
}

function decompress(s: string): number {
  if (s.length === 0) return 1

  let sum = 0
  for(let i=0; i<s.length; ++i) {
    if (s[i] === '(') {
      const marker = getMarker(s, i)
      i += marker.length + 2
      const [size, repetitions] = marker.split('x').map(v => Number(v))
      sum += repetitions * decompress(s.slice(i, i+size))
      i += size - 1
    } else {
      sum += 1
    }
  }

  return sum
}

function solver2(input: string) {
  let sum = 0
  
  for(const line of input.split('\r\n')) {
    sum += decompress(line)
  }

  return sum
} 

import execute from './handler'
execute(solver2, 'day09.txt')