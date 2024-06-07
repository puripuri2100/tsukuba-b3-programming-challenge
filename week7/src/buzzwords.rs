use std::collections::HashMap;

pub fn main() {
  loop {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let s = s.trim();
    if s.is_empty() {
      break;
    }
    let s = s.split_whitespace().collect::<String>();
    let len = s.len();
    let mut start_pos_map: HashMap<String, Vec<usize>> = std::collections::HashMap::new();
    for (i, c) in s.chars().enumerate() {
      if let Some(l) = start_pos_map.get_mut(&c.to_string()) {
        l.push(i);
      } else {
        start_pos_map.insert(c.to_string(), vec![i]);
      }
    }
    for plus_n in 1..=len {
      let mut new_start_pos_map: HashMap<String, Vec<usize>> = std::collections::HashMap::new();
      if start_pos_map.is_empty() {
        break;
      }
      let mut max_n = 0_usize;
      for (_, lst) in start_pos_map.iter() {
        let len2 = lst.len();
        if len2 > 1 {
          max_n = max_n.max(len2);
          for start_pos in lst.iter() {
            if start_pos + plus_n < len {
              let sub = &s[*start_pos..=*start_pos + plus_n];
              if let Some(l) = new_start_pos_map.get_mut(sub) {
                l.push(*start_pos);
              } else {
                new_start_pos_map.insert(sub.to_string(), vec![*start_pos]);
              }
            }
          }
        }
      }
      if max_n > 1 {
        println!("{max_n}");
      } else {
        break;
      }
      start_pos_map = new_start_pos_map;
    }
    println!()
  }
}
