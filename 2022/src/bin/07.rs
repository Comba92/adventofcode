use std::collections::HashMap;

#[derive(Debug)]
struct Dir<'a> {
  files: HashMap<&'a str, usize>,
  children: Vec<&'a str>
}
impl<'a> Dir<'a> {
  fn new() -> Self {
    Dir {files: HashMap::new(), children: Vec::new() }
  }
}

fn compute_sizes(dir: &Dir, dirs: &HashMap<&str, Dir>, sizes: &mut Vec<usize>) -> usize {
  let mut size: usize = dir.files.values().sum();

  for child in &dir.children {
    size += compute_sizes(dirs.get(child).unwrap(), dirs, sizes);
  }

  sizes.push(size);
  size
}

fn main() {
  let input = include_str!("07.txt");

  let mut dirs: HashMap<&str, Dir>= HashMap::new();
  let root = Dir::new();
  dirs.insert("/", root);
  let mut stack = vec!["/"];

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
        stack.push(dir);

        if !dirs.contains_key(dir) {
          dirs.insert(dir, Dir::new());
        }
      },

      ["dir", subdir] => {
        dirs.insert(subdir, Dir::new());

        let dir = dirs.get_mut(stack.last().unwrap()).unwrap();
        dir.children.push(subdir);
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

  
  let mut sizes = Vec::new();
  compute_sizes(dirs.get("/").unwrap(), &dirs, &mut sizes);
  println!("Total dirs: {}, Sizes: {}, / childs: {}", dirs.len(), sizes.len(), dirs.get("/").unwrap().children.len());
  
  println!("{:?}", dirs);
  println!("{:?}", sizes);
  let result1: usize = sizes.iter().filter(|&&size| size < 100000).sum();

  println!("{result1}");
}