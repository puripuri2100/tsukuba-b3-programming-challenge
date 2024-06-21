#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct T {
  h: usize,
  start: usize,
  start_base: usize,
  end: usize,
  end_base: usize,
}

pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  let n = s.trim().parse::<usize>().unwrap();
  let mut v = Vec::new();
  for _ in 0..n {
    s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut s_l = s.split_whitespace();
    let h = s_l.next().unwrap().parse().unwrap();
    let start = s_l.next().unwrap().parse().unwrap();
    let end = s_l.next().unwrap().parse().unwrap();
    let t = T {
      h,
      start,
      end,
      start_base: 0,
      end_base: 0,
    };
    v.push(t);
  }

  let len = v.len();

  // 基礎になりうるか検査する
  for i in 0..len {
    let t = v[i];
    // 対象の床
    #[allow(clippy::needless_range_loop)]
    for j in 0..len {
      let t2 = v[j];
      if t.h < t2.h {
        // 開始位置
        if t.h > t2.start_base && t.start <= t2.start && t2.start < t.end {
          v[j].start_base = t.h;
        }
        // 終了位置
        if t.h > t2.end_base && t.start < t2.end && t2.end <= t.end {
          v[j].end_base = t.h;
        }
      }
    }
  }

  let mut ans = 0_usize;
  for t in v.iter() {
    ans += t.h - t.start_base;
    ans += t.h - t.end_base;
  }
  println!("{ans}");
}
