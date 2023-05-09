function isPossibleTriangle(triangle: number[]) {
  return triangle[0] + triangle[1] > triangle[2] &&
         triangle[1] + triangle[2] > triangle[0] &&
         triangle[2] + triangle[0] > triangle[1]     
}

function solver(input: string, counter: (list: number[][]) => number) {  
  let triangles: number[][] = []
  for(const line of input.split('\r\n')) {
    triangles.push(line.trim().split('  ').map(p => Number(p)))
  }

  return counter(triangles)
}

function trianglesByRows(triangles: number[][]) {
  let possibleTriangles = 0
  for(const triangle of triangles) {
    if (isPossibleTriangle(triangle)) 
      possibleTriangles += 1
  }

  return possibleTriangles
}

function trianglesByColumns(triangles: number[][]) {
  let possibleTriangles = 0
  let row = 0, col = 0

  while(col < 3) {
    let triangle: number[] = []
    triangle.push(triangles  [row][col])
    triangle.push(triangles[row+1][col])
    triangle.push(triangles[row+2][col])
    console.log(triangle)
    if (isPossibleTriangle(triangle)) 
      possibleTriangles += 1

    row = row + 3
    if (row >= triangles.length) {
      row = 0
      col = col + 1
    }
  }

  return possibleTriangles
}

import execute from './handler'
execute((input) => solver(input, trianglesByColumns), 'day03.txt')