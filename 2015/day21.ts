export interface Stats {
  hp: number,
  damage: number,
  armor: number
}

interface Item {
  name: string,
  type: string,
  cost: number,
  damage: number,
  armor: number
}

function parseItems(input: string): Item[] {
  const headers = ['Weapons:', 'Armor:', 'Rings:', '']
  
  let items: Item[] = []
  let currentType: string = ''
  for(const line of input.split('\r\n')) {
    const tokens = line.split(/\s\s+/g)
    if (headers.includes(tokens[0])) {
      currentType = tokens[0].replace(':', '')
      continue
    }
    
    const name = tokens[0]
    const cost = Number(tokens[1])
    const damage = Number(tokens[2])
    const armor = Number(tokens[3])
    items.push({name, type: currentType, cost, damage, armor})
  }

  return items
}

function computeRingsCombinations(rings: Item[]) {
  let combinations: [Item,Item][] = []

  for(let i=0; i<rings.length; ++i) {
    for(let j=i+1; j<rings.length; ++j) {
      combinations.push([rings[i], rings[j]])
    }
  }

  return combinations
}


export function canKillBoss(player: Stats, boss: Stats): boolean {
  const playerDmg = Math.max(player.damage - boss.armor, 1)
  const bossDmg = Math.max(boss.damage - player.armor, 1)

  return Math.ceil(boss.hp / playerDmg) <= Math.ceil(player.hp / bossDmg)
}

/* 
  Damage is at least 1
  Exactly one weapon
  Armos is optional, but not more than one
  0 to 2 rings
  Any item bought must be used
  There are no duplicate items
  Kill the boss and minimize the gold
*/

let playerStats: Stats = {
  hp: 100, damage: 0, armor: 0
}

const bossStats: Stats = {
  hp: 104, damage: 8, armor: 1
}


function minAmountOfGoldToWin(weapons: Item[], armors: Item[], ringsPairs: [Item,Item][]) {
  let minGoldSpent = Number.MAX_SAFE_INTEGER
  for(const weapon of weapons) {
    for(const armor of armors) {
      for(const pair of ringsPairs) {
        let player: Stats = {
          hp: playerStats.hp, damage: weapon.damage, armor: armor.armor
        }
        let goldSpent = weapon.cost + armor.cost

        for(const ring of pair) {
          player.damage += ring.damage
          player.armor += ring.armor
          goldSpent += ring.cost
        }

        if (canKillBoss(player, bossStats))
          minGoldSpent = Math.min(goldSpent, minGoldSpent)
      }
    }
  }

  return minGoldSpent
}

function maxAmountOfGoldToLose(weapons: Item[], armors: Item[], ringsPairs: [Item,Item][]) {
  let maxGoldSpent = 0
  for(const weapon of weapons) {
    for(const armor of armors) {
      for(const pair of ringsPairs) {
        let player: Stats = {
          hp: playerStats.hp, damage: weapon.damage, armor: armor.armor
        }
        let goldSpent = weapon.cost + armor.cost

        for(const ring of pair) {
          player.damage += ring.damage
          player.armor += ring.armor
          goldSpent += ring.cost
        }

        if (!canKillBoss(player, bossStats))
          maxGoldSpent = Math.max(goldSpent, maxGoldSpent)
      }
    }
  }

  return maxGoldSpent
}

function solver(input: string, condition: (weapons: Item[], armors: Item[], ringsPairs: [Item,Item][]) => number) {
  const items = parseItems(input)
  items.push(
    {name: 'None', type: 'Armor', cost: 0, damage: 0, armor: 0},
    {name: 'None Left', type: 'Rings', cost: 0, damage: 0, armor: 0},
    {name: 'None Right', type: 'Rings', cost: 0, damage: 0, armor: 0}
  )

  const weapons = items.filter(i => i.type === 'Weapons')
  const armors = items.filter(i => i.type === 'Armor')
  const rings = items.filter(i => i.type === 'Rings')
  const ringsPairs = computeRingsCombinations(rings)

  return condition(weapons, armors, ringsPairs)
}

import execute from './handler'
execute((input) => solver(input, maxAmountOfGoldToLose), 'day21.txt')