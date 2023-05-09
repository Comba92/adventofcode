import fs from 'fs'

for(let i=1; i<=25; ++i) {
  const paddedDay = (i).toString().padStart(2, '0')
  const code = 
    `function solver(input: string) {}\n\nimport execute from './handler'\nexecute(solver, 'day` + paddedDay + `.txt')`
  
  fs.writeFile('day'+ paddedDay +'.ts', code, () => {})
  fs.writeFile('day'+ paddedDay +'.txt', '', () => {})
}
