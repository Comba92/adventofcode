import { readFileSync } from "fs"

export default function execute(solver: (input: string) => any, inputPath: string, ) {
  const input = readFileSync(inputPath).toString().trim()
  console.log(solver(input))
}