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

  let mut diff = 0;
  let mut diff_i = 0;
  for i in 2..45 {
    let m = ((1 + i) * i) / 2 - 1;
    if n <= m {
      break;
    } else {
      diff = n - m;
      diff_i = i;
    }
  }
  // dp[diff_i][diff]: diff_i回まで先に進んで後戻りをした回数がdiff回数のときの最小値
  let mut dp = vec![vec![0_usize; diff]; diff_i];
  for i in 0..diff_i {
    for j in 0..diff {
      if i > j {
        let v = dp[j - 1][i - 1];
        let value = get_value(&lst, i, j);
        dp[j][i] = v + value;
      } else if i == j {
        let value = get_value(&lst, i, j);
      }
    }
  }
  println!("{diff_i}: {diff}");
}

// 進んだ回数と下がった回数から価格を取得する
fn get_value(lst: &[usize], up: usize, down: usize) -> usize {
  lst[((1 + up) * up) / 2 + down - 1]
}
