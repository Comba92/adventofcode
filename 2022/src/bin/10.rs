struct Cpu {
  cycle: usize,
  x: i32,
  crt: [[bool; 40]; 6]
}


impl Cpu {
  fn tick_and_check(&mut self, sum: &mut i32) {
    let row = self.cycle / 40;
    let col = (self.cycle % 40);

    if (self.x-1 ..= self.x+1).contains(&(col as i32)) {
      self.crt[row][col] = true;
    }
    
    self.cycle += 1;
    if (self.cycle >= 20) && ((self.cycle - 20) % 40 == 0) {
      *sum += self.x * self.cycle as i32
    }
  }

  fn draw(&self) {
    for row in self.crt {
      for col in row {
        print!("{}", if col {'#'} else {'.'})
      }
      println!()
    }
    println!()
  }
}


fn main() {
  let input = include_str!("10.txt");
  
  let mut cpu = Cpu {cycle: 0, x: 1, crt: [[false; 40]; 6]};

  let mut result1 = 0;
  for line in input.lines() {
    let instruction = line.split_whitespace().collect::<Vec<_>>();

    match instruction.as_slice() {
      ["noop"] => { 
        cpu.tick_and_check(&mut result1);
      },
      ["addx", value] => {
        cpu.tick_and_check(&mut result1);
        cpu.tick_and_check(&mut result1);
        cpu.x += value.parse::<i32>().unwrap();
      },
      _ => unreachable!()
    }
  }
  
  cpu.draw();
  println!("{result1}");
}