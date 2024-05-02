pub fn main() {
  let mut s = String::new();
  if std::io::stdin().read_line(&mut s).is_err() {
    return;
  }
  if s.trim().is_empty() {
    return;
  }
  let x = s.trim().parse::<usize>().unwrap();
  for _ in 0..x {
    let mut s = String::new();
    if std::io::stdin().read_line(&mut s).is_err() {
      return;
    }
    if s.trim().is_empty() {
      return;
    }
    let n = s.trim().parse::<usize>().unwrap();
    let mut order = Vec::new();
    let mut stock = Vec::new();
    for _ in 0..n {
      let mut s = String::new();
      if std::io::stdin().read_line(&mut s).is_err() {
        return;
      }
      if s.trim().is_empty() {
        return;
      }
      let mut s_lst = s.trim().split_ascii_whitespace();
      let cmd = s_lst.next().unwrap();
      let x = s_lst.next().unwrap().parse::<usize>().unwrap();
      s_lst.next();
      s_lst.next();
      let y = s_lst.next().unwrap().parse::<usize>().unwrap();
      if cmd == "buy" {
        order.push((x, y));
        // 高い順
        order.sort_by(|a, b| b.1.cmp(&a.1))
      }
      if cmd == "sell" {
        stock.push((x, y));
        // 高い順
        stock.sort_by(|a, b| a.1.cmp(&b.1))
      }
      let mut new_order = Vec::new();
      let mut result_value = None;
      // 取引の実施
      for (order_x, order_y) in order.iter() {
        println!("{order_x} {order_y}:  {stock:?}");
        let mut new_stock = Vec::new();
        // 購入量
        let mut order_x = *order_x;
        for (stock_x, stock_y) in stock.iter() {
          if order_y >= stock_y && order_x != 0 {
            result_value = Some(*stock_y);
            if order_x > *stock_x {
              order_x = order_x - stock_x;
            } else if order_x == *stock_x {
              order_x = 0;
            } else {
              new_stock.push((stock_x - order_x, *stock_y));
              order_x = 0;
            }
          } else {
            new_stock.push((*stock_x, *stock_y));
          }
        }
        if order_x != 0 {
          new_order.push((order_x, *order_y));
        }
        stock = new_stock;
      }
      order = new_order;

      println!(
        "{} {} {}",
        stock
          .iter()
          .min_by_key(|a| a.1)
          .map(|v| v.1.to_string())
          .unwrap_or("-".to_string()),
        order
          .iter()
          .max_by_key(|a| a.1)
          .map(|v| v.1.to_string())
          .unwrap_or("-".to_string()),
        result_value
          .map(|n| n.to_string())
          .unwrap_or("-".to_string())
      )
    }
  }
}
