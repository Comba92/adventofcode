// https://stackoverflow.com/questions/52356091/recursive-algorithm-to-generate-all-permutations-of-length-k-of-a-list-in-python
export function computeSubsets<T>(list: T[], size: number) {
  function backtrack(curr: number, size: number) {
    if (size <= 0) return [[]]

    let subsets: T[][] = []
    for (let i = curr+1; i < list.length; i++) {
      const subset = backtrack(i, size-1)
      for (const p of subset)
        subsets.push([list[i], ...p])
    }
  
    return subsets
  }

  return backtrack(-1, size)
}

export function computeSubsetsWithSum(list: number[], sum: number) {
  function backtrack(curr: number, sum: number) {
    if (sum === 0) return [[]]

    let subsets: number[][] = []
    for (let i = curr+1; i < list.length; i++) {
      if (sum - list[i] < 0) continue
      const subset = backtrack(i, sum - list[i])
      for (const p of subset)
        subsets.push([list[i], ...p])
    }
  
    return subsets
  }

  return backtrack(-1, sum)
}

export function computePermutations<T>(list: T[]) { 
  let perms: T[][] = []

  function backtrack(sublist: T[]) {
    if (sublist.length === list.length) {
      perms.push(sublist)
      return 
    }

    for (let i = 0; i < list.length; i++) {
      if (sublist.includes(list[i])) continue
      sublist.push(list[i])
      backtrack([...sublist])
      sublist.pop()
    }
  }

  backtrack([])
  return perms
}

export function range(a: number, b: number) {
  let r: number[] = []
  for (let i=a; i<=b; i++) {
    r.push(i)
  }
  return r
}