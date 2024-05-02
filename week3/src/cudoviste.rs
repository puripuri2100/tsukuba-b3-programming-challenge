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
    r: usize,
    c: usize,
    clst: [chars; r]
  }

  let mut cnt0 = 0_usize;
  let mut cnt1 = 0_usize;
  let mut cnt2 = 0_usize;
  let mut cnt3 = 0_usize;
  let mut cnt4 = 0_usize;
  for i in 0..(r - 1) {
    for j in 0..(c - 1) {
      let mut n = 0;
      let c1 = clst[i][j];
      let c2 = clst[i][j + 1];
      let c3 = clst[i + 1][j];
      let c4 = clst[i + 1][j + 1];
      if c1 != '#' && c2 != '#' && c3 != '#' && c4 != '#' {
        if c1 == 'X' {
          n += 1;
        }
        if c2 == 'X' {
          n += 1;
        }
        if c3 == 'X' {
          n += 1;
        }
        if c4 == 'X' {
          n += 1;
        }
        if n == 0 {
          cnt0 += 1;
        } else if n == 1 {
          cnt1 += 1;
        } else if n == 2 {
          cnt2 += 1;
        } else if n == 3 {
          cnt3 += 1;
        } else if n == 4 {
          cnt4 += 1;
        }
      }
    }
  }

  println!("{}", cnt0);
  println!("{}", cnt1);
  println!("{}", cnt2);
  println!("{}", cnt3);
  println!("{}", cnt4);
}
