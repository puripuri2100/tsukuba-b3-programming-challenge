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
    m: usize,
    k: usize,
    n_lst: [usize; n],
    m_lst: [usize; m],
    k_lst: [usize; k],
  }

  let mut r_size_sort_lst = m_lst.iter().map(|n| *n as f64).collect::<Vec<_>>();
  for k in k_lst.iter() {
    let f = *k as f64 * 0.7; // √2 / 2の近似値
    r_size_sort_lst.push(f);
  }
  // 半径がでかい順に並べる
  r_size_sort_lst.sort_by(|a, b| a.partial_cmp(b).unwrap());

  let mut n_lst = n_lst;
  n_lst.sort_by(|a, b| a.cmp(b));
  let mut anum = n_lst.len() - 1;
  let mut bnum = r_size_sort_lst.len() - 1;
  let mut cnt = 0_usize;
  // でかいやつから入れるように探索する
  loop {
    let plots_r = n_lst[anum];
    let house_r = r_size_sort_lst[bnum];
    //println!("plots: {plots_r}({anum}), house: {house_r}({bnum})");
    if plots_r as f64 > house_r {
      // 入るやつが見つかったら土地と家を両方消費する
      cnt += 1;
      if anum == 0 {
        break;
      }
      anum -= 1;
      if bnum == 0 {
        break;
      }
      bnum -= 1;
    } else {
      // 家の方がでかかったら家を小さくする
      if bnum == 0 {
        break;
      }
      bnum -= 1;
    }
  }

  println!("{}", cnt);
}
