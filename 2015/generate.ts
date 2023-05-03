import fs from 'fs'

for(let i=1; i<=25; ++i) {
  const paddedDay = (i).toString().padStart(2, '0')
  const code = 
    `import execute from './handler'\nexecute(, 'day` + paddedDay + `.txt')`
  
  fs.writeFile('day'+ paddedDay +'.ts', code, () => {})
  fs.writeFile('day'+ paddedDay +'.txt', '', () => {})
}
