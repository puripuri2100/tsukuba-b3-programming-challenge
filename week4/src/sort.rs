// (c) tanakah
// <https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8>
macro_rules! input {
  (source = $s:expr, $($r:tt)*) => {
      let mut iter = $s.split_whitespace();
      let mut next = || { iter.next().unwrap() };
      input_inner!{next, $($r)*}
  };
  ($($r:tt)*) => {
      let stdin = std::io::stdin();
      let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
      let mut next = move || -> String{
          bytes
              .by_ref()
              .map(|r|r.unwrap() as char)
              .skip_while(|c|c.is_whitespace())
              .take_while(|c|!c.is_whitespace())
              .collect()
      };
      input_inner!{next, $($r)*}
  };
}

macro_rules! input_inner {
  ($next:expr) => {};
  ($next:expr, ) => {};

  ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
      let $var = read_value!($next, $t);
      input_inner!{$next $($r)*}
  };
}

macro_rules! read_value {
  ($next:expr, ( $($t:tt),* )) => {
      ( $(read_value!($next, $t)),* )
  };

  ($next:expr, [ $t:tt ; $len:expr ]) => {
      (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
  };

  ($next:expr, chars) => {
      read_value!($next, String).chars().collect::<Vec<char>>()
  };

  ($next:expr, usize1) => {
      read_value!($next, usize) - 1
  };

  ($next:expr, $t:ty) => {
      $next().parse::<$t>().expect("Parse error")
  };
}

#[derive(Debug, Clone, Copy)]
struct RangeInfo {
  start: usize,
  end: usize,
  len: usize
}

impl RangeInfo {
  fn new_range(&self, w: usize) -> Self {
    if w < self.start {
      Self {
        start: w,
        end: self.end,
        len: self.len +1
      }
    } else if self.end < w {
      Self {
        start: self.start,
        end: w,
        len: self.len + 1
      }
    } else {
      *self
    }
  }

  // 幅が小さい方を出す
  fn cmp(&self, other: &Self) -> Self {
    if self.start <= other.start && self.end >= other.end {
      *other
    } else {
      *self
    }
  }
}

pub fn main() {
  input! {
    n: usize,
    lst: [usize; n],
  }

  // 現在の最大の長さ
  // 探索範囲の枝刈できる
  let mut max_len = 1_usize;
  // dp[i]: 長さがiの組み合わせのうち、最も小さい幅の組み合わせ
  let mut dp = vec![None; n + 1];
  dp[1] = Some(RangeInfo {
    start: lst[0],
    end: lst[0],
    len: 1
  });
  for (i, weight) in lst.iter().enumerate() {
    if i != 0 {
      for l in 0..=max_len {
        if let Some(range) = dp[l] {
          let new_range = range.new_range(*weight);
          if let Some(range) = dp[l + 1] {
            let r = range.cmp(&new_range);
            if r.len == l + 1 {
              dp[l + 1] = Some(r);
            }
          } else {
            dp[new_range.len] = Some(new_range);
            if l == max_len {
              max_len = new_range.len;
            }
          }
        }
      }
    }
    //println!("{dp:?}");
  }

  println!("{max_len}");
}
