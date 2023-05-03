# adventOfCode
My Advent of Code solutions. Learn more at https://adventofcode.com

# Running the 2015 problems
The 2015 problmes are solved with Typescript.
Install ts-node
```bash
npm install -g typescript
npm install -g ts-node
```
and run a script like so:
```bash
ts-node {day}.ts
```

In every script there is an execute function, which takes a function and a string of the input data filename. As a single day of Advent of Code
has multiple problems, just change the first argument of the execute function with the desired solver function.
