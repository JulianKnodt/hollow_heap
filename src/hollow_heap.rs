use std::collections::VecDeque;

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct NodePtr(usize);

#[derive(Debug, Default)]
pub struct HollowHeap<K, V> {
  root: usize,
  nodes: Vec<Node<K, V>>,
  len: usize,
  empty_slots: Vec<usize>,
}

impl<K: PartialOrd, V: PartialEq> HollowHeap<K, V> {
  pub fn new() -> Self {
    HollowHeap {
      root: 0,
      len: 0,
      nodes: vec![],
      empty_slots: vec![],
    }
  }
  pub fn peek(&self) -> Option<&V> { self.nodes.get(self.root)?.value.as_ref() }
  pub fn push(&mut self, k: K, v: V) -> NodePtr {
    let ptr = self.insert_node(Node::new(k, v));
    self.len += 1;
    ptr
  }
  pub fn len(&self) -> usize { self.len }
  pub fn is_empty(&self) -> bool { self.len == 0 }
  pub fn decrease_root_key(&mut self, k: K) {
    if self.is_empty() {
      return;
    }
    let root = &mut self.nodes[self.root];
    assert!(root.key >= k);
    root.key = k;
  }
  pub fn delete(&mut self, NodePtr(i): NodePtr) -> Option<(&K, V)> {
    if i == self.root {
      self.pop()
    } else {
      let out = self.nodes[i].hollow()?;
      self.len -= 1;
      Some((&self.nodes[i].key, out))
    }
  }
  pub fn pop(&mut self) -> Option<(&K, V)> {
    let out = self.nodes.get_mut(self.root)?.value.take()?;
    self.len -= 1;
    let mut current_roots = vec![self.root];
    let mut non_empty_roots = vec![];
    while let Some(root) = current_roots.pop() {
      let node = &mut self.nodes[root];
      if node.is_hollow() {
        self.empty_slots.push(root);
        current_roots.extend(node.children.drain(..));
      } else {
        let r = node.rank as usize;
        if non_empty_roots.len() <= r {
          non_empty_roots.resize_with(r + 1, Vec::new);
        }
        non_empty_roots[r].push(root);
      }
    }
    if non_empty_roots.is_empty() {
      assert_eq!(self.len, 0);
      return Some((&self.nodes[self.root].key, out));
    }
    let mut i = 0;
    while i < non_empty_roots.len() {
      while non_empty_roots[i].len() > 1 {
        let rs = &mut non_empty_roots[i];
        let a = rs.pop().unwrap();
        let b = rs.pop().unwrap();
        while non_empty_roots.len() <= i + 1 {
          non_empty_roots.push(vec![]);
        }
        non_empty_roots[i + 1].push(self.link(a, b));
      }
      i += 1;
    }
    debug_assert!(non_empty_roots
      .iter()
      .map(|it| it.len())
      .all(|l| l == 0 || l == 1));
    let mut roots = non_empty_roots.drain(..).filter_map(|mut it| it.pop());
    let curr = roots.next().unwrap();
    let og_root = self.root;
    self.root = roots.fold(curr, |acc, n| self.link(acc, n));
    Some((&self.nodes[og_root].key, out))
  }
  /// Decreases the key of an element and returns a pointer to the new node
  pub fn decrease_key(&mut self, k: K, NodePtr(i): NodePtr) -> NodePtr {
    if i == self.root {
      self.decrease_root_key(k);
      return NodePtr(self.root);
    }
    let node = &mut self.nodes[i];
    let item = if let Some(item) = node.hollow() {
      item
    } else {
      return NodePtr(i);
    };
    let mut new_node = Node::new(k, item);
    new_node.rank = node.rank.saturating_sub(2);
    new_node
      .children
      .extend(node.children.drain(new_node.rank as usize..));
    self.insert_node(new_node)
  }
  fn insert_node(&mut self, node: Node<K, V>) -> NodePtr {
    let idx = if let Some(reuse) = self.empty_slots.pop() {
      // TODO maybe we don't need to construct a node here or smth?
      assert!(self.nodes[reuse].value.is_none());
      self.nodes[reuse] = node;
      reuse
    } else {
      let idx = self.nodes.len();
      self.nodes.push(node);
      idx
    };
    self.root = if self.is_empty() {
      idx
    } else {
      self.link(self.root, idx)
    };
    NodePtr(idx)
  }

  // returns true if a was the winner
  fn link(&mut self, a: usize, b: usize) -> usize {
    assert_ne!(a, b);
    let an = &self.nodes[a];
    let bn = &self.nodes[b];
    if an.rank == bn.rank {
      let (h, l) = if an.key < bn.key { (a, b) } else { (b, a) };
      self.nodes[h].add_last_child(l);
      self.nodes[h].rank += 1;
      h
    } else {
      let (w, l) = if an.key < bn.key { (a, b) } else { (b, a) };
      self.nodes[w].add_first_child(l);
      w
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
struct Node<K, V> {
  // key is priority
  key: K,
  // value is item
  value: Option<V>,

  // which are the children of this node
  children: VecDeque<usize>,

  // rank of this node
  rank: u32,
}

impl<K: PartialOrd, V: PartialEq> Node<K, V> {
  fn new(key: K, value: V) -> Self {
    Self {
      key,
      value: Some(value),
      rank: 0,
      children: VecDeque::new(),
    }
  }
  fn hollow(&mut self) -> Option<V> { self.value.take() }
  fn is_hollow(&mut self) -> bool { self.value.is_none() }
  fn add_last_child(&mut self, idx: usize) { self.children.push_back(idx); }
  fn add_first_child(&mut self, idx: usize) { self.children.push_front(idx); }
}
