use std::u32;

#[derive(Debug)]
struct Emulator {
  a: u64,
  b: u64,
  c: u64,
  ip: usize,
}
impl Emulator {
  fn combo_operand(&self, operand: u8) -> u64 {
    match operand {
      0..=3 => operand as u64,
      4 => self.a,
      5 => self.b,
      6 => self.c,
      _ => unreachable!("illegal combo operand")
    }
  }

  fn run(&mut self, program: &[u8]) -> String {
    let mut output = Vec::new();
    loop {
      if self.ip+1 >= program.len() { break; }

      let opcode = program[self.ip];
      let operand = program[self.ip + 1];
      self.ip += 2;

      match opcode {
        0 => {
          // adv
          let den = self.combo_operand(operand);
          self.a = self.a >> den;
        }
        1 => {
          // bxl
          self.b = self.b ^ operand as u64;
        }
        2 => {
          // bst
          let modulo = self.combo_operand(operand) & 0b111;
          self.b = modulo;
        }
        3 => {
          // jnz
          if self.a != 0 {
            self.ip = operand as usize;
          }
        }
        4 => {
          // bxc
          self.b = self.b ^ self.c;
        }
        5 => {
          // out
          let modulo = self.combo_operand(operand) & 0b111;
          output.push(modulo as u8);
        }
        6 => {
          // bdv
          let den = self.combo_operand(operand);
          self.b = self.a >> den;
        }
        7 => {
          // cdv
          let den = self.combo_operand(operand);
          self.c = self.a >> den;
        }

        _ => unreachable!("illegal opcode")
      }
    }

    output.iter()
      .map(|n| n.to_string())
      .collect::<Vec<_>>()
      .join(",")

    // let mut res = output.iter()
    //   .fold(String::new(), |mut s, &n| {
    //     s.push(char::from_u32(n as u32).unwrap());
    //     s.push(',');
    //     s
    //   });

    // res.pop();
    // res
  }
}

fn main() {
  let input = include_str!("17.txt");
  let mut lines = input.lines();
  
  let parse_reg_value = |line: &str| {
    line.chars()
      .skip_while(|c| !c.is_numeric())
      .collect::<String>()
      .parse::<u64>()
      .unwrap()
  };
  
  let a = parse_reg_value(lines.next().unwrap());
  let b = parse_reg_value(lines.next().unwrap());
  let c = parse_reg_value(lines.next().unwrap());

  // empty line
  lines.next();
  let program_line = lines.next().unwrap();
  let program_start = program_line
    .find(|c: char| c.is_numeric())
    .unwrap();

  let program = program_line[program_start..]
    .split(',')
    .map(|c| c.parse::<u8>().unwrap())
    .collect::<Vec<_>>();

  let program_str = program.iter()
    .map(|n| n.to_string())
    .collect::<Vec<_>>()
    .join(",");

  println!("{a} {b} {c}");
  println!("{program:?}");
  let mut emu = Emulator { a, b, c, ip: 0 };

  let res1 = emu.run(&program);
  println!("{res1}");

  // https://github.com/rene-d/advent-of-rust/blob/e085ae984972fe01ec024e22c93baec7f46eef92/src/year2024/day17/day17.rs#L153
  
  // 'search: for i in 0.. {
  //   emu = Emulator { a: i, b, c, ip: 0 };
  //   let mut op_count = 0;

  //   'emulation: loop {
  //     if op_count >= program.len() {
  //       println!("Found A! {i}");
  //       break 'search;
  //     }

  //     emu.b = emu.a & 7;
  //     emu.b = emu.b ^ 3;
  //     emu.c = emu.a >> emu.b;
  //     emu.b = emu.b ^ emu.c;
  //     emu.a = emu.a >> 8;
  //     emu.b = emu.b ^ 5;
  //     if (emu.b & 7) as u8 != program[op_count] || emu.a == 0 {
  //       break 'emulation;
  //     }
  //     op_count += 1;
  //   }
  // }

  // for i in 8u64.pow(16)..8u64.pow(16) + 10000 {
  // for i in 0.. {
  //   let i = (i << 13) | 0b0101010111111;
  //   emu = Emulator {a: i, b: 0, c: 0, ip: 0};
  //   let res = emu.run(&program);
  //   // println!("{i} {i:x} {i:o} {i:b}: {res}");
  //   if res == program_str {
  //     println!("{i}");
  //     break;
  //   }

  //   let i = (i << 13) | 0b1101010100000;
  //   emu = Emulator {a: i, b: 0, c: 0, ip: 0};
  //   let res = emu.run(&program);
  //   // println!("{i} {i:x} {i:o} {i:b}: {res}");
  //   if res == program_str {
  //     println!("{i}");
  //     break;
  //   }

  //   let i = (i << 13) | 0b1101010111111;
  //   emu = Emulator {a: i, b: 0, c: 0, ip: 0};
  //   let res = emu.run(&program);
  //   // println!("{i} {i:x} {i:o} {i:b}: {res}");
  //   if res == program_str {
  //     println!("{i}");
  //     break;
  //   }
  // }
}

// b <- a % 8 = a & 7
// b <- b ^ 3
// c <- a / 2**b
// b <- b ^ c
// a <- a / 8 = a >> 3
// b <- b ^ 5
// print(b % 8) = (b & 7)
// repeat to 0

// 2751 0101010111111
// 6816 1101010100000
// 6847 1101010111111

