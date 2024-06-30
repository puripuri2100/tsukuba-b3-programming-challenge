#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Gear {
  x: isize,
  y: isize,
  r: usize,
  l: usize,
}

pub fn main() {
  loop {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    if s.trim().is_empty() {
      break;
    }
    let n = s.trim().parse::<usize>().unwrap();
    let mut v = Vec::new();
    let mut graph = vec![Vec::new(); n];

    for i in 0..n {
      let mut s = String::new();
      std::io::stdin().read_line(&mut s).unwrap();
      let mut s_l = s.split_whitespace();
      let x = s_l.next().unwrap().parse::<isize>().unwrap();
      let y = s_l.next().unwrap().parse::<isize>().unwrap();
      let r = s_l.next().unwrap().parse::<usize>().unwrap();
      let gear = Gear { x, y, r, l: 0 };
      for j in 0..i {
        let gear2: Gear = v[j];
        let len =
          x.abs_diff(gear2.x) * x.abs_diff(gear2.x) + y.abs_diff(gear2.y) * y.abs_diff(gear2.y);
        if len == (r + gear2.r) * (r + gear2.r) {
          graph[i].push(j);
          graph[j].push(i);
        }
      }
      v.push(gear);
    }
    if search(0, n, &graph, &mut v, 1) {
      if v[n - 1].l == 0 {
        // targetに接続できていない
        println!("0")
      } else {
        // 回転数を割り出す
        let gcd = gcd(v[0].r, v[n - 1].r);
        let start = v[n - 1].r / gcd;
        let mut target = (v[0].r / gcd) as isize;
        if v[n - 1].l % 2 != 0 {
          target *= -1;
        }
        println!("{start} {target}");
      }
    } else {
      // 回転できない
      println!("-1");
    }
  }
}

fn search(start: usize, end: usize, graph: &[Vec<usize>], v: &mut [Gear], n: usize) -> bool {
  for i in graph[start].iter() {
    if v[*i].l == 0 {
      // まだ訪れたことが無い
      v[*i].l = n;
      if *i != end && !search(*i, end, graph, v, n + 1) {
        // 伝播
        return false;
      }
    } else {
      // 訪れたことがあるので回転数の向きチェック
      if v[*i].l % 2 != n % 2 {
        // 回転の向きがあっていない
        return false;
      }
    }
  }
  true
}

fn gcd(mut m: usize, mut n: usize) -> usize {
  if m < n {
    gcd(n, m)
  } else {
    let mut r: usize;
    loop {
      r = m % n;
      if r == 0 {
        return n;
      }
      m = n;
      n = r;
    }
  }
}
