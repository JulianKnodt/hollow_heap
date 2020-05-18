use crate::hollow_heap::HollowHeap;

use quickcheck::{Arbitrary, Gen};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Actions<K, V> {
  Pop,
  Push(K, V),
  // DecreaseKey(V, K),
}

impl<K: Ord, V: PartialEq> Actions<K, V> {
  pub fn apply_hollow(self, heap: &mut HollowHeap<K, V>) -> Option<(&K, V)> {
    match self {
      Actions::Pop => heap.pop(),
      Actions::Push(k, v) => {
        heap.push(k, v);
        None
      },
    }
  }
  pub fn apply_bin(self, heap: &mut BinaryHeap<Reverse<K>>) -> Option<Reverse<K>> {
    match self {
      Actions::Pop => heap.pop(),
      Actions::Push(k, _) => {
        heap.push(Reverse(k));
        None
      },
    }
  }
}

impl<K: Ord + Arbitrary> Arbitrary for Actions<K, ()> {
  fn arbitrary<G: Gen>(g: &mut G) -> Self {
    if bool::arbitrary(g) {
      Actions::Pop
    } else {
      Actions::Push(K::arbitrary(g), ())
    }
  }
}

quickcheck! {
  fn match_bin(acts: Vec<Actions<i32, ()>>) -> bool {
    let mut bh = BinaryHeap::new();
    let mut hh = HollowHeap::new();
    for a in acts {
      let expected = a.apply_bin(&mut bh);
      let got = a.apply_hollow(&mut hh);
      assert_eq!(expected.map(|e| e.0), got.map(|g| *g.0));
    }
    true
  }
}
