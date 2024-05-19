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

pub fn main() {
  input! {
    n: usize,
    lst: [usize; n],
  }

  // dp[i][j]：jマスにジャンプ幅iで行くときの入場料の最小合計
  let mut dp = vec![vec![None; n + 1]; n + 1];
  // スタート地点なので当然入場料0
  dp[0][1] = Some(0);
  // 答え
  let mut a = usize::MAX;

  for i in 1..n {
    // 最大のジャンプ幅
    // 長さの和が最大
    let jump_max = n.min(((i * (i + 1)) / 2) + 1);

    // 前進するときのコスト
    for j in (i + 1)..=jump_max {
      if let Some(t) = dp[i - 1][j - i] {
        dp[i][j] = Some(t + lst[j - 1]);
      }

      // 目的地に到達できたときに答えを更新
      if j == n {
        if let Some(t) = dp[i][j] {
          a = a.min(t);
        }
      }
    }

    // 後退した場合のコストの更新
    for k in (1..=jump_max - i).rev() {
      if let Some(t1) = dp[i][k] {
        if let Some(t2) = dp[i][k + i] {
          dp[i][k] = Some(t1.min(t2 + lst[k - 1]));
        }
      } else {
        if let Some(t2) = dp[i][k + i] {
          dp[i][k] = Some(t2 + lst[k - 1]);
        }
      }
    }
  }

  println!("{a}");
}
