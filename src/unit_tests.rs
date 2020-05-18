use crate::hollow_heap::HollowHeap;

#[test]
fn simple() {
  let mut h = HollowHeap::new();
  h.push(-1, "d");
  h.push(2, "o");
  h.push(1, "a");
  h.push(3, "b");
  h.push(0, "c");
  assert_eq!(h.peek(), Some(&"d"));
}

#[test]
fn rm() {
  let mut h = HollowHeap::new();
  h.push(2, "o");
  h.push(1, "a");
  h.push(3, "b");
  h.push(-1, "d");
  let to_rm = h.push(0, "c");
  h.delete(to_rm);
  assert_eq!(h.peek(), Some(&"d"));
}

#[test]
fn pop_min() {
  let mut h = HollowHeap::new();
  h.push(-1, "d");
  h.push(2, "o");
  h.push(1, "a");
  h.push(3, "b");
  assert_eq!(h.pop().map(|t| t.1), Some("d"));
  assert_eq!(h.pop().map(|t| t.1), Some("a"));
  assert_eq!(h.pop().map(|t| t.1), Some("o"));
  assert_eq!(h.pop().map(|t| t.1), Some("b"));
  assert_eq!(h.pop().map(|t| t.1), None);
  assert_eq!(h.len(), 0);
}

#[test]
fn decrease_key() {
  let mut h = HollowHeap::new();
  h.push(-1, "d");
  let ptr = h.push(2, "o");
  h.push(1, "a");
  h.push(3, "b");
  h.decrease_key(-3, ptr);
  assert_eq!(h.pop().map(|t| t.1), Some("o"));
  assert_eq!(h.pop().map(|t| t.1), Some("d"));
  assert_eq!(h.pop().map(|t| t.1), Some("a"));
  assert_eq!(h.pop().map(|t| t.1), Some("b"));
  assert_eq!(h.pop().map(|t| t.1), None);
  assert_eq!(h.len(), 0);
}
