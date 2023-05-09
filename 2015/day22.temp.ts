interface State {
  player: {hp: number, mana: number, armor: number},
  boss: {hp: number, damage: number},
  castedSpells: Spell[]
}

// Can't cast a spell while it is active. However, they can be casted on the same turn they end
// Activates/deactivates immediately
interface ActiveSpell {
  name: string
  cost: number
  effect: (s: State) => State
  turns: number
  type: 'Active'
}

// They get actived next turn
interface PassiveSpell {
  name: string
  cost: number
  activate: (s: State) => State
  deactivate: (s: State) => State
  turns: number
  type: 'Passive'
}

type Spell = ActiveSpell | PassiveSpell

const bossAttack: Spell = {
  name: 'Boss Attack',
  cost: 0,
  type: 'Active',
  turns: 0,
  effect: (s: State) => {
    const newPlayer = {...s.player, hp: s.player.hp - Math.max(s.boss.damage - s.player.armor, 1) }
    return {...s, player: newPlayer}
  }
}

const spells: Spell[] = [
 {
      name: 'Magic Missile',
      cost: 53,
      type: 'Active',
      turns: 0,
      effect: (s: State) => {
        const newBoss = {...s.boss, hp: s.boss.hp - 4}
        return {...s, boss: newBoss}
      }

  }, {
      name: 'Drain',
      cost: 73,
      type: 'Active',
      turns: 0,
      effect: (s: State) => {
        const newBoss = {...s.boss, hp: s.boss.hp - 2}
        const newPlayer = {...s.player, hp: s.player.hp + 2}
        return {...s, player: newPlayer, boss: newBoss}
      }
      
  }, {
      name: 'Shield',
      cost: 113,
      type: 'Passive',
      turns: 6,
      activate: (s: State) => {
        const newPlayer = {...s.player, armor: s.player.armor + 7}
        return {...s, player: newPlayer}
      },
      deactivate: (s: State) => {
        const newPlayer = {...s.player, armor: s.player.armor - 7}
        return {...s, player: newPlayer}
      }
  }, {
    name: 'Poison',
    cost: 173,
    type: 'Active',
    turns: 6,
    effect: (s: State) => {
      const newBoss = {...s.boss, hp: s.boss.hp - 3}
      return {...s, boss: newBoss}
    }
  }, {
    name: 'Recharge',
    cost: 229,
    type: 'Active',
    turns: 5,
    effect: (s: State) => {
      const newPlayer = {...s.player, mana: s.player.mana + 101}
      return {...s, player: newPlayer}
    }
  }
]

const bossStats = {
  hp: 55, damage: 8
}

const playerStats = {
  hp: 50, armor: 0, mana: 500
}

function nextState(s: State, spellChosen: Spell): State {
  let newSpells: Spell[] = []
  
  // decrease effects timer
  for(const casted of s.castedSpells) {
    let newSpell = {...casted, turns: casted.turns - 1}
    
    if (newSpell.turns <= 0) {
      // if turns are 0, deactivate the passive
      if (casted.type === 'Passive') {
        s = casted.deactivate(s)
      // if turns are 0, run effect one last time
      } else if (casted.type === 'Active') {
        s = casted.effect(s)
      }

    // if turns aren't 0, add to new spells
    } else {
      newSpells.push(newSpell)

      // active spells must be casted each turn
      if (casted.type === 'Active') {
        s = casted.effect(s)
      }
    }
  }

  // Active is casted and first effects takes place next turn
  // Passive is activated immediately
  if (spellChosen.type === 'Passive')
    s = spellChosen.activate(s)

  newSpells.push({...spellChosen})
  const newPlayer = {...s.player, mana: s.player.mana - spellChosen.cost}

  return {...s, castedSpells: newSpells, player: newPlayer}
}

function debug(s: State, spell: Spell, turn: number) {
  console.log('Turn', turn.toString().padStart(3, '0'), '-- Player:', s.player, ' \t\tBoss:', s.boss, '\tSpell:', spell.name, s.castedSpells.map(s => s.name))
}

function getMinManaSpent(s: State, turn=1): number {
  if (s.player.hp <= 0) return Number.MAX_SAFE_INTEGER
  if (s.boss.hp <= 0) return 0

  let minManaSpent = Number.MAX_SAFE_INTEGER
  for(const chosenSpell of spells) {
    // don't cast spell if already active
    if (s.castedSpells.find(s => s.name === chosenSpell.name) || s.player.mana < chosenSpell.cost) continue

    let newS = {...s}

    newS = nextState(newS, chosenSpell)
/*     if (newS.boss.hp <= 0) console.log('boss dead')
    else debug(newS, chosenSpell, turn) */
    if (newS.player.hp <= 0 || newS.player.mana <= 0) return Number.MAX_SAFE_INTEGER
    if (newS.boss.hp <= 0) return 0

    newS = nextState(newS, bossAttack)
/*     if (newS.player.hp <= 0 || newS.player.mana <= 0) console.log('player dead')
    else debug(newS, bossAttack, turn+1) */
    if (newS.player.hp <= 0 || newS.player.mana <= 0) return Number.MAX_SAFE_INTEGER
    if (newS.boss.hp <= 0) return 0

    const manaSpent = chosenSpell.cost + getMinManaSpent(newS, turn+2)
    minManaSpent = Math.min(minManaSpent, manaSpent)
  }

  return minManaSpent
}

const initialState = {player: playerStats, boss: bossStats, castedSpells: []}
console.log(getMinManaSpent(initialState))
