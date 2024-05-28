#[derive(Debug, Clone)]
struct Info {
  n: usize,
  to: Vec<usize>,
  to_size: usize,
  from: Vec<usize>,
  from_size: usize,
  group: usize,
}

pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let case = s.trim().parse::<usize>().unwrap();
  for _ in 0..case {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut s_l = s.split_whitespace();
    let _n = s_l.next();
    let m = s_l.next().unwrap().parse::<usize>().unwrap();
    s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut group_info = std::collections::VecDeque::new();
    for (n, s) in s.split_whitespace().enumerate() {
      let v = s.parse::<usize>().unwrap();
      group_info.push_back(Info {
        n,
        to: Vec::new(),
        to_size: 0,
        from: Vec::new(),
        from_size: 0,
        group: v,
      });
    }
    for _ in 0..m {
      s = String::new();
      std::io::stdin().read_line(&mut s).unwrap();
      let mut s_l = s.split_whitespace();
      let i = s_l.next().unwrap().parse::<usize>().unwrap();
      let j = s_l.next().unwrap().parse::<usize>().unwrap();
      group_info[i - 1].from.push(j - 1);
      group_info[i - 1].from_size += 1;
      group_info[j - 1].to.push(i - 1);
      group_info[j - 1].to_size += 1;
    }
    println!("{group_info:?}");

    while let Some(info) = group_info.pop_front() {}
  }
}
