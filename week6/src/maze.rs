// https://docs.rs/num-integer/0.1.46/src/num_integer/lib.rs.html#870
// (c) The Rust Project Developers
// MIT License
fn gcd(mut m: usize, mut n: usize) -> usize {
  // Use Stein's algorithm
  if m == 0 || n == 0 {
    return m | n;
  }

  // find common factors of 2
  let shift = (m | n).trailing_zeros();

  // divide n and m by 2 until odd
  m >>= m.trailing_zeros();
  n >>= n.trailing_zeros();

  while m != n {
    if m > n {
      m -= n;
      m >>= m.trailing_zeros();
    } else {
      n -= m;
      n >>= n.trailing_zeros();
    }
  }
  m << shift
}

pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let n = s.trim().parse::<usize>().unwrap();
  let mut lst = Vec::new();
  for _ in 0..n {
    s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let v = s.trim().parse::<usize>().unwrap();
    lst.push(v);
  }
  lst.sort();

  let mut graph = vec![Vec::new(); n];
  for i in 0..n - 1 {
    for j in i + 1..n {
      let gcd_num = gcd(lst[i], lst[j]);
      if gcd_num > 1 {
        // 互いの位置も記録する
        let lenj = graph[j].len();
        let leni = graph[i].len();
        graph[i].push((j, gcd_num, lenj));
        graph[j].push((i, gcd_num, leni));
      }
    }
  }

  let mut ans = 0_usize;
  loop {
    let mut check = vec![false; n];
    let d = f(0, n - 1, usize::MAX, &mut check, &mut graph);
    if d == 0 {
      break;
    } else {
      ans += d;
    }
  }

  println!("{ans}");
}

fn f(
  from: usize,
  target: usize,
  value: usize,
  check: &mut Vec<bool>,
  graph: &mut Vec<Vec<(usize, usize, usize)>>,
) -> usize {
  if from == target {
    value
  } else {
    check[from] = true;
    for index in 0..graph[from].len() {
      let (to, weight, link) = graph[from][index];
      if !check[to] && weight > 0 {
        let d = f(to, target, value.min(weight), check, graph);
        if d > 0 {
          if weight < d {
            graph[from][index].1 = 0;
          } else {
            graph[from][index].1 -= d;
          }
          graph[to][link].1 += d;
          return d;
        }
      }
    }
    0
  }
}
