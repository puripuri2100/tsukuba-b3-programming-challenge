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
    lst: [[usize; 2]; n],
  }


  let mut lst = lst.iter().map(|l| (l[0], l[1])).collect::<Vec<_>>();

  //lst.sort_by(cmp);
  lst.sort();
  let mut cnt = 1;
  let mut lst_iter = lst.iter().peekable();
  let mut tmp = *lst_iter.next().unwrap();
  while let Some(next) = lst_iter.next() {
    let (_, tmp_e) = tmp;
    let (next_s, next_e) = next;
    if tmp_e < *next_s {
      // 被ってないので終了
      tmp = *next;
      cnt += 1;
    } else {
      tmp = (*next_s, tmp_e.min(*next_e));
    }
  }
  println!("{}", cnt);
}
