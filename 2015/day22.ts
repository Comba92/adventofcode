/* 
  if you cant cast, you lose
  when you cast, cost is instantly deducted
  effects all apply at the start of the turn
  after effects are applied, the timer decreases
  if the timer is 0, the effect ends
  you cant cast a spell already active
*/

const playerStats = {
  hp: 50, mana: 500, armor: 0
}

const bossStats = {
  hp: 55, dmg: 8
}

interface State {
  player: {hp: number, mana: number, armor: number},
  boss: {hp: number, dmg: number},
  effects: Spell[]
}

interface ActiveSpell {
  name: string
  cost: number
  apply: (s: State) => void
  turns: number
  type: 'Active'
}

interface PassiveSpell {
  name: string
  cost: number
  apply: (s: State) => void
  disable: (s: State) => void
  turns: number
  type: 'Passive'
}

type Spell = ActiveSpell | PassiveSpell

const spells: Spell[] = [
 {
      name: 'Magic Missile',
      cost: 53,
      type: 'Active',
      turns: 0,
      apply: (s: State) => { s.boss.hp -= 4 }
  }, {
      name: 'Drain',
      cost: 73,
      type: 'Active',
      turns: 0,
      apply: (s: State) => { s.boss.hp -= 2; s.player.hp += 2 }
  }, {
      name: 'Shield',
      cost: 113,
      type: 'Passive',
      turns: 6,
      apply: (s: State) => { s.player.armor += 7 },
      disable: (s: State) => { s.player.armor -= 7 }
  }, {
    name: 'Poison',
    cost: 173,
    type: 'Active',
    turns: 6,
    apply: (s: State) => { s.boss.hp -= 3 }
  }, {
    name: 'Recharge',
    cost: 229,
    type: 'Active',
    turns: 5,
    apply: (s: State) => { s.player.mana += 101 }
  }
]

function handleActiveEffects(s: State) {
  for(const effect of s.effects) {
    if (effect.type === 'Active') {
      effect.apply(s)
    }

    effect.turns -= 1

    if (effect.type === 'Passive' && effect.turns === 0) {
      effect.disable(s)
    }
  }
  s.effects = s.effects.filter(e => e.turns > 0)
}

function handlePlayerTurn(s: State, spellToCast: Spell): number {  
  handleActiveEffects(s)
  
  if (s.boss.hp <= 0) return 0

  if (spellToCast.type === 'Passive') spellToCast.apply(s)

  if (spellToCast.turns === 0) spellToCast.apply(s)
  else s.effects.push({...spellToCast})

  s.player.mana = s.player.mana - spellToCast.cost
  return spellToCast.cost
}

function handleBossTurn(s: State){
  handleActiveEffects(s)

  if (s.boss.hp <= 0) return
  s.player.hp = s.player.hp - Math.max(s.boss.dmg - s.player.armor, 1)
}

function prompt(s: State, spell: Spell, turn: number) {
  console.log(`TURN ${turn}`)
  
  if (turn % 2 === 0) {
    console.log('-- Boss Turn --')
  } else console.log('-- Player Turn --')

  if (s.player.hp <= 0) {
    console.log('Player dead!')
    return
  }
  if (s.boss.hp <= 0) {
    console.log('Boss dead!')
    return
  }

  console.log(`- Player has ${s.player.hp} hp, ${s.player.armor} armor, ${s.player.mana} mana`)
  console.log(`- Boss has ${s.boss.hp} hp.`)
  
  for(const effect of s.effects) {
    console.log(`${effect.name} casted. ${effect.turns} uses remains.`)
  }

  if (turn % 2 === 0) {
    console.log(`Boss attacks for ${s.boss.dmg}`)
  } else console.log(`Player casts ${spell.name}`)
}

function cloneState(s: State): State {
  let effects: Spell[] = []
  for(const e of s.effects) effects.push({...e})

  return {player: {...s.player}, boss: {...s.boss}, effects}
}

function getMinManaSpent(s: State, hardMode=false) {
  let minManaSpent = Number.MAX_SAFE_INTEGER

  function simulateNextTurn(s: State, previousManaSpent=0, turn=1) {
    for(const spellToCast of spells) {
      if (s.effects.find(e => e.name === spellToCast.name)) continue
      if (s.player.mana - spellToCast.cost <= 0) continue

      let newState = cloneState(s)

      if (hardMode) {
        newState.player.hp = newState.player.hp - 1
        if (newState.player.hp <= 0) continue
      }

      const currentManaSpent = handlePlayerTurn(newState, spellToCast)
      const newManaSpent = previousManaSpent + currentManaSpent
      if (newManaSpent > minManaSpent) continue
      
      if (newState.boss.hp <= 0) {
        minManaSpent = Math.min(newManaSpent, minManaSpent)
        continue
      }

      handleBossTurn(newState)
      if (newState.boss.hp <= 0) {
        minManaSpent = Math.min(newManaSpent, minManaSpent)
        continue
      }
      if (newState.player.hp <= 0) continue

      simulateNextTurn(newState, newManaSpent, turn+2)
    }
  }

  simulateNextTurn(s)
  return minManaSpent
}

function solver() {
  let s: State = {player: {...playerStats}, boss: {...bossStats}, effects: []}

  console.log(new Date())
  console.log(getMinManaSpent(s, true))
  console.log(new Date())
}

solver()