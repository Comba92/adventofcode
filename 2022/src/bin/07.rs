use std::{cell::Cell, cmp::Reverse, collections::{BinaryHeap, HashMap}, path::PathBuf};

#[derive(Debug)]
struct Dir<'a> {
  size: Cell<usize>,
  files: HashMap<&'a str, usize>,
  children: Vec<PathBuf>
}
impl<'a> Dir<'a> {
  fn new() -> Self {
    Dir {size: Cell::new(0), files: HashMap::new(), children: Vec::new() }
  }
}

fn compute_sizes(dir: &Dir, dirs: &HashMap<PathBuf, Dir>) -> usize {
  let mut size: usize = dir.files.values().sum();

  for child in &dir.children {
    size += compute_sizes(dirs.get(child).unwrap(), dirs);
  }

  dir.size.set(size);
  size
}

fn main() {
  let input = include_str!("07.txt");

  let mut dirs: HashMap<PathBuf, Dir>= HashMap::new();
  let root_path = PathBuf::from("/");

  let root = Dir::new();
  let mut stack = vec![root_path.clone()];
  dirs.insert(root_path.clone(), root);

  for line in input.lines() {
    let tokens = line.split_whitespace().collect::<Vec<_>>();

    match tokens.as_slice() {
      ["$", "ls"] => continue,
      ["$", "cd", "/"] => {
        stack.drain(1..);
      },
      ["$", "cd", ".."] => {
        stack.pop();
      },
      ["$", "cd", dir] => {
        let mut dir_path = stack.last().unwrap().clone();
        dir_path.push(dir);
        stack.push(dir_path.clone());

        if !dirs.contains_key(&dir_path) {
          dirs.insert(dir_path, Dir::new());
        }
      },

      ["dir", subdir] => {
        // dirs.insert(subdir, Dir::new());

        let dir = dirs.get_mut(stack.last().unwrap()).unwrap();
        let mut subdir_path = stack.last().unwrap().clone();
        subdir_path.push(subdir);
        dir.children.push(subdir_path);
      },

      [file_size, file_name] => {
        let dir = dirs.get_mut(stack.last().unwrap()).unwrap();
        if !dir.files.contains_key(file_name) {
          dir.files.insert(file_name, file_size.parse::<usize>().unwrap());
        }
      },
      _ => {},
    }
  }
  
  let root = dirs.get(&root_path).unwrap();
  compute_sizes(&root, &dirs);

  let sizes = dirs.values().map(|dir| dir.size.get());
  let result1: usize = sizes.clone().filter(|&size| size < 100000).sum();

  let target_size = 30000000 - (70000000 - root.size.get());
  let result2 = BinaryHeap::from_iter(
    sizes.filter(|&size| size > target_size).map(|size| Reverse(size))
  ).peek().unwrap().0;

  println!("{result1} {result2}");
}