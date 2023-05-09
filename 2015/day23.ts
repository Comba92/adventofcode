class Program {
  registers: Map<string, number>
  ip: number
  instructions: Instruction[]

  constructor(instructions: Instruction[]) {
    this.registers = new Map([['a', 0], ['b', 0]])
    this.ip = 0
    this.instructions = instructions
  }

  execute() {
    if (this.ip < 0 || this.ip >= this.instructions.length)
      throw new Error(`Instruction Pointer is out of bounds, with value ${this.ip}!`)

    const i = this.instructions[this.ip]
    switch(i.type) {
      case 'Register':
        switch(i.name) {
          case 'hlf': this.hlf(i); break
          case 'tpl': this.tpl(i); break
          case 'inc': this.inc(i); break
        }
        break

      case 'Value': this.jmp(i); break
      case 'RegisterValue':
        switch(i.name) {
          case 'jie': this.jie(i)
          case 'jio': this.jio(i)
        }
        break
    }

    this.ip = this.ip + 1
  }

  debug() {
    console.log(this.registers, this.ip)
    console.log(this.instructions[this.ip])
  }

  hlf(i: RegisterI) {
    const val = this.registers.get(i.register)!
    this.registers.set(i.register, Math.floor(val / 2))
  }

  tpl(i: RegisterI) {
    const val = this.registers.get(i.register)!
    this.registers.set(i.register, val*3)
  }

  inc(i: RegisterI) {
    const val = this.registers.get(i.register)!
    this.registers.set(i.register, val+1)
  }

  jmp(i: ValueI) {
    this.ip = this.ip + i.offset - 1
  }

  jie(i: RegisterValueI) {
    if (this.registers.get(i.register)! % 2 === 0) {
      this.ip = this.ip + i.offset - 1
    }
  }

  jio(i: RegisterValueI) {
    if (this.registers.get(i.register)! === 1) {
      this.ip = this.ip + i.offset - 1
    }
  }
}

type Instruction = RegisterI | ValueI | RegisterValueI
interface RegisterI {
  type: 'Register'
  name: string
  register: string
}
interface ValueI {
  type: 'Value'
  name: string
  offset: number
}
interface RegisterValueI {
  type: 'RegisterValue'
  name: string
  register: string
  offset: number
}

function parseInstruction(instruction: string): Instruction {
  const tokens = instruction.split(' ')

  const name = tokens[0]
  if(name[0] === 'j') {
    if (tokens.length === 2) {
      const offset = Number(tokens[1])
      return {type: 'Value', name, offset}
    }

    const register = tokens[1].replace(',', '')
    const offset = Number(tokens[2])

    return {type: 'RegisterValue', name, register, offset}
  }

  const register = tokens[1]
  return {type: 'Register', name, register}
}

function solver(input: string) {
  const instructions: Instruction[] = []
  for(const line of input.split('\r\n')) {
    const i = parseInstruction(line)
    instructions.push(i)
  }

  const computer = new Program(instructions)
  computer.registers.set('a', 1)
  try {
    while(true) {
      computer.execute()
    }
  } catch(e) { if(e instanceof Error) console.log(e.message) }

  return computer.registers.get('b')
}

import execute from './handler'
execute(solver, 'day23.txt')