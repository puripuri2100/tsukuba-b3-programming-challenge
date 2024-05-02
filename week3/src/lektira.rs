pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).ok();
  let s = s.trim();
  let len = s.len();
  let chars = s.chars().collect::<Vec<_>>();
  let mut v = Vec::new();
  for i in 0..len - 2 {
    for j in i + 1..len - 1 {
      let mut s = String::new();
      for s1 in 0..=i {
        s.push(chars[i - s1]);
      }
      for s2 in 0..=(j - i - 1) {
        s.push(chars[j - s2]);
      }
      for s3 in 0..(len - j - 1) {
        s.push(chars[len - s3 - 1])
      }
      v.push(s);
    }
  }
  v.sort();
  println!("{}", v[0]);
}

/*
pub fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).ok();
  let chars = s.trim().chars().collect::<Vec<_>>();
  let len = chars.len();
  let mut chars_iter = chars.iter().enumerate().peekable();
  let max_char1 = chars[0..len - 2].iter().min().unwrap();
  let mut s1 = Vec::new();
  loop {
    if let Some((i, c)) = chars_iter.next() {
      if c == max_char1 {
        s1.push(*c);
        if i == len - 3 {
          break;
        }
        if Some(max_char1) != chars_iter.peek().map(|v| v.1) {
          break;
        }
      } else {
        s1.push(*c);
      }
    }
  }
  let chars2 = chars_iter.clone().collect::<Vec<_>>();
  let len2 = chars2.len();
  let max_char2 = *chars2[0..len2 - 1].to_vec().iter().map(|v| v.1).min().unwrap();
  let mut s2 = Vec::new();
  loop {
    if let Some((i, c)) = chars_iter.next() {
      if *c == max_char2 {
        s2.push(*c);
        if i == len - 2 {
          break;
        }
        if Some(&max_char2) != chars_iter.peek().map(|v| v.1) {
          break;
        }
      } else {
        s2.push(*c);
      }
    }
  }
  let mut s3 = Vec::new();
  for (_, c) in chars_iter {
    s3.push(*c);
  }
  s1.reverse();
  let s1 = s1.iter().collect::<String>();
  s2.reverse();
  let s2 = s2.iter().collect::<String>();
  s3.reverse();
  let s3 = s3.iter().collect::<String>();
  println!("{}{}{}", s1, s2, s3)
}
*/
