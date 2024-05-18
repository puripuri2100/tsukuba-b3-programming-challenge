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
  let mut dp = vec![vec![usize::MAX; n + 1]; n + 1];
  // スタート地点なので当然入場料0
  dp[0][1] = 0;
  // 答え
  let mut a = usize::MAX;

  for i in 1..n {
    // 最大のジャンプ幅
    // 長さの和が最大
    let jump_max = n.min(((i * (i + 1)) / 2) + 1);

    println!("-- {i}: {jump_max}");

    // 前進するときのコスト
    for j in (i + 1)..=jump_max {
      dp[i][j] = dp[i - 1][j - i] + lst[j - 1];
      println!("dp[{i}][{j}]: {}", dp[i][j]);

      if j == n {
        a = a.min(dp[i][j]);
      }
    }

    // 後退した場合のコストの更新
    for k in (1..=jump_max - 1).rev() {
      println!("k: {k}");
      dp[i][k] = (dp[i][k]).min(dp[i][k + 1] + lst[k - 1]);
    }
    println!("a: {a}");
    println!("dp: {dp:?}");
  }

  println!("{a}");
}
