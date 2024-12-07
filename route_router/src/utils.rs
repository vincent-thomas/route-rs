use std::collections::VecDeque;

pub(crate) fn insert_at<T>(l: &mut VecDeque<T>, idx: usize, val: T) {
  let mut tail = l.split_off(idx);
  l.push_back(val);
  l.append(&mut tail);
}
