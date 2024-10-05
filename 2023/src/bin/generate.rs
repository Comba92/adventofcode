use std::{fs::File, io::Write, path::PathBuf};

fn main() {
  for i in 1..=25 {
    let name_rs = format!("{i:0<2}.rs");
    let name_txt = format!("{i:0<2}.txt");
    let path_txt = PathBuf::from_iter(["src/bin", &name_txt]);
    let path_rs = PathBuf::from_iter(["src/bin", &name_rs]);

    _ = File::create_new(path_txt).inspect_err(|e| eprintln!("{e}"));
    
    let file = File::create_new(path_rs).inspect_err(|e| eprintln!("{e}"));
    if let Ok(mut file) = file {
      let content = format!(
"fn main() {{
  let input = include_str!(\"{i:0<2}.txt\");
}}");
      _ = file.write(content.as_bytes()).inspect_err(|e| eprint!("{e}"));
    }
  }
}