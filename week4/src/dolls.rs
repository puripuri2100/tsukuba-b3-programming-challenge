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
  l.sort_by(|a, b| b.0.cmp(&a.0));
  let mut new_l = Vec::new();
  let mut count = 1;
  loop {
    let mut now_w = l[0].0;
    let mut now_h = l[0].1;
    let mut l_iter = l.iter();
    l_iter.next().unwrap();
    for (w, h) in l_iter {
      if now_w > *w && now_h > *h {
        now_w = *w;
        now_h = *h;
      } else {
        new_l.push((*w, *h));
      }
    }

    if new_l.is_empty() {
      break;
    }
    *l = new_l.clone();
    new_l = Vec::new();
    count += 1;
  }
  count
}
