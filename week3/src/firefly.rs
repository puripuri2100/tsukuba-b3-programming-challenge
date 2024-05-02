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
  input!{
    n: usize,
    h: usize,
    lst: [usize; n],
  }

  let mut even_lst = Vec::new();
  let mut odd_lst = Vec::new();

  for (i, l) in lst.iter().enumerate() {
    if i % 2 == 0 {
      even_lst.push(*l);
    } else {
      odd_lst.push(*l);
    }
  }

  even_lst.sort();
  odd_lst.sort();

  let mut even_sum_lst = vec![0; h];
  let mut h_tmp = 0_usize;
  let mut h_count = 0;
  for h in even_lst.iter() {
    if h_tmp == 0 {
      h_tmp = *h;
      h_count = 1;
    } else if h_tmp == *h {
      h_count += 1;
    } else {
      h_tmp = *h;
      h_count = 1;
    }
  }

  //println!("{} {}", min, count);
}
