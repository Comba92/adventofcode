function getID(s: string): number {
  return Number(s.slice(-(CHECKSUM_LEGTH + ID_LENGTH), -CHECKSUM_LEGTH))
}

function getCheckSum(s: string): string {
  return s.slice(-(CHECKSUM_LEGTH-1), -1)
}

function getName(s: string): string {
  return s.slice(0, -(ID_LENGTH + CHECKSUM_LEGTH + 1))
}

function computeCheckSum(s: string): string {
    const name = getName(s)
    let repetitions = new Array(26).fill(0)

    // Count occurences
    for(const letter of name) {
      if (letter === '-') continue
      repetitions[letter.charCodeAt(0) - 'a'.charCodeAt(0)] += 1
    }

    // Get array as [letter, count] and sort by count
    repetitions = repetitions
      .map((v, i) => {
        return {
          letter: String.fromCharCode('a'.charCodeAt(0) + i),
          count: v
        }
      })
      .sort((a, b) => b.count-a.count)

    // build checksum by taking the 5 most common letters
    return repetitions.slice(0, 5).map(l => l.letter).join('')
}

const ID_LENGTH = 3
const CHECKSUM_LEGTH = 7

function solver1(input: string) {
  let sum = 0
  for(const line of input.split('\r\n')) {
    if (computeCheckSum(line) === getCheckSum(line)) {
      sum += getID(line)
    }
  }

  return sum
}

function decrypt(s: string) {
  const id = getID(s)
  let decrypted = ''

  for(const letter of s) {
    if (!isNaN(Number(letter))) break
    if (letter === '-') {
      decrypted += ' '
      continue
    }
    const decryptedCharCode = (letter.charCodeAt(0) - 'a'.charCodeAt(0) + id) % 26 
    decrypted += String.fromCharCode(decryptedCharCode + 'a'.charCodeAt(0))
  }

  return {str: decrypted.trim(), id}
}

function solver2(input: string) {
  const targets = ['north', 'pole', 'objects', 'object']

  for(const line of input.split('\r\n')) {
    for(const target of targets) {
      const decrypted = decrypt(line)
      if (decrypted.str.includes(target)) {
        return decrypted.id
      }
    }
  }
}

import execute from './handler'
execute(solver2, 'day04.txt')