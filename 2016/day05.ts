import md5 from 'md5'

function solver1(input: string) {
  let password = ''
  let i = 0
  while(password.length < 8) {
    const hash = md5(input + i.toString())
    if (hash.slice(0, 5) === '00000') {
      password += hash[5]
      console.log(`${password.length} character found with index ${i}!`)
    }
    if (i % (10 ** 6) == 0) console.log(i / (10 ** 6) + ' million hashes computed...')
    i += 1
  }

  return password
}

function solver2(input: string) {
  let password = new Array(8).fill('_')
  let i = 0
  let found = new Set<number>()
  while(found.size < 8) {
    const hash = md5(input + i.toString())
    const index = Number(hash[5])
    if ( 
      hash.slice(0, 5) === '00000' && 
      !isNaN(index) && !found.has(index) && index < 8
    ) {
      password[index] = hash[6]
      found.add(index)
      console.log(`${found.size} characters found with index ${i}! String is: ${password.join('')}`)
    }
    if (i % (10 ** 6) == 0) console.log(i / (10 ** 6) + ' million hashes computed...')
    i += 1
  }

  return password.join('')
}

import execute from './handler'
execute(solver2, 'day05.txt')