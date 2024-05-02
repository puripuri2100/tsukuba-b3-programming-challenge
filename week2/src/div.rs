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
    divided: String, // 割られる数
    div: String, // 割る数
  }
  // 割られる数と割る数がそれぞれ10の何乗か
  // 10 -> 1
  // 100 -> 2
  let divided_len = divided.len() - 1;
  let div_len = div.len() - 1;
  if divided_len >= div_len {
    // 割られる数のほうが大きい
    let divided_chars = divided.chars().collect::<Vec<_>>();
    let (left, right) = divided_chars.split_at(divided_len - div_len + 1);
    let mut right = right.to_vec();
    right.reverse();
    let mut right2 = Vec::new();
    let mut is_zero = true;
    for c in right.iter() {
      if is_zero {
        if *c != '0' {
          is_zero = false;
          right2.push(*c);
        }
      } else {
        right2.push(*c)
      }
    }
    right2.reverse();
    if right2.is_empty() {
      println!("{}", left.iter().collect::<String>())
    } else {
      println!(
        "{}.{}",
        left.iter().collect::<String>(),
        right2.iter().collect::<String>()
      )
    }
  } else {
    // 割る数のほうが大きいので小数
    let mut divided_chars = divided.chars().collect::<Vec<_>>();
    divided_chars.reverse();
    let mut is_zero = true;
    let mut divided_str = Vec::new();
    for c in divided_chars.iter() {
      if is_zero {
        if *c != '0' {
          divided_str.push(*c);
          is_zero = false;
        }
      } else {
        divided_str.push(*c);
      }
    }
    divided_str.reverse();
    let zero_s = "0".repeat(div_len - divided_len - 1);
    println!("0.{}{}", zero_s, divided_str.iter().collect::<String>());
  }
}
