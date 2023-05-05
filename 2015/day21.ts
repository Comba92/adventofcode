interface Stats {
  hp: number,
  damage: number,
  armor: number
}

type Type = 'Weapons' | 'Armor' | 'Rings'

interface Item {
  name: string,
  type: Type,
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

function canKillBoss(player: Stats, boss: Stats): boolean {
  const playerDmg = Math.max(player.damage - boss.armor, 1)
  const bossDmg = Math.max(boss.damage - player.armor, 1)

  return Math.floor(boss.hp / playerDmg) <= Math.floor(player.hp / bossDmg)
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

function solver(input: string) {
  const items = parseItems(input)
  const weapons = items.filter(i => i.type === 'Weapons')
  const armor = items.filter(i => i.type === 'Armor')
  const rings = items.filter(i => i.type === 'Rings')

  let playerStats: Stats = {
    hp: 100, damage: 0, armor: 0
  }
  let bossStats: Stats = {
    hp: 104, damage: 8, armor: 1
  }

  let goldSpent = 0
}

import execute from './handler'
execute(solver, 'day21.txt')