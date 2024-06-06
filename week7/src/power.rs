pub fn main() {
  loop {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let s = s.trim();
    if s == "." {
      break;
    }
    let len = s.len();
    let mut is_ok = false;
    for i in 1..len {
      if len % i == 0 {
        // 約数
        let base_s = &s[0..i];
        is_ok = true;
        for n in 1..=len / i {
          if base_s != &s[(n - 1) * i..n * i] {
            is_ok = false;
            break;
          }
        }
        if is_ok {
          println!("{}", len / i);
          break;
        }
      }
    }
    if !is_ok {
      println!("1")
    }
  }
}
