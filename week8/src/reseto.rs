pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let mut s_l = s.split_whitespace();
  let n = s_l.next().unwrap().parse::<usize>().unwrap();
  let k = s_l.next().unwrap().parse::<usize>().unwrap();
  let mut range = (2..=n).collect::<Vec<usize>>();
  let mut count = 0_usize;
  loop {
    let mut new_range = Vec::new();
    let p = range.first().unwrap();
    for v in range.iter() {
      if v % p == 0 {
        count += 1;
        if count == k {
          println!("{v}");
          return;
        }
      } else {
        new_range.push(*v)
      }
    }
    range = new_range;
  }
}
