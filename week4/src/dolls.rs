pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let t = s.trim().parse::<usize>().unwrap();
  for _ in 0..t {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let m = s.trim().parse::<usize>().unwrap();
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut s_l = s.trim().split_whitespace();
    let mut l = Vec::new();
    for _ in 0..m {
      let w = s_l.next().unwrap().parse::<usize>().unwrap();
      let h = s_l.next().unwrap().parse::<usize>().unwrap();
      l.push((w, h));
    }

    let a = f(&mut l);
    println!("{a}");
  }
}

fn f(l: &mut Vec<(usize, usize)>) -> usize {
  l.sort();

  println!("{l:?}");

  let mut lis = Vec::new();

  for (_w, h) in l.iter() {
    if let Err(pos) = lis.binary_search(h) {
      if pos == lis.len() {
        lis.push(*h);
      } else {
        lis[pos] = *h
      }
    }
  }

  lis.len()
}
