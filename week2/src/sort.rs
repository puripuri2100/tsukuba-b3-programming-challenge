fn sort(a: &str, b: &str) -> std::cmp::Ordering {
  let mut a_chars = a.chars();
  let mut b_chars = b.chars();
  let a1 = a_chars.next();
  let b1 = b_chars.next();
  match (a1, b1) {
    (Some(a1), Some(b1)) => {
      let ord1 = a1.cmp(&b1);
      if ord1 == std::cmp::Ordering::Equal {
        let a2 = a_chars.next();
        let b2 = b_chars.next();
        match (a2, b2) {
          (Some(a2), Some(b2)) => a2.cmp(&b2),
          (None, None) => std::cmp::Ordering::Equal,
          (None, _) => std::cmp::Ordering::Less,
          (_, None) => std::cmp::Ordering::Greater,
        }
      } else {
        ord1
      }
    }
    (None, None) => std::cmp::Ordering::Equal,
    (None, _) => std::cmp::Ordering::Less,
    (_, None) => std::cmp::Ordering::Greater,
  }
}

pub fn main() {
  loop {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let n = s.trim().parse::<usize>().unwrap();
    if n == 0 {
      break;
    }
    let mut name_lst = Vec::new();
    for _ in 0..n {
      let mut s = String::new();
      std::io::stdin().read_line(&mut s).ok();
      let s = s.trim().to_string();
      name_lst.push(s);
    }
    let mut name_lst = name_lst;
    name_lst.sort_by(|a, b| sort(a, b));
    for v in name_lst.iter() {
      println!("{}", v);
    }
    println!("");
  }
}
