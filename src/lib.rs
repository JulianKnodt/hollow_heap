pub mod hollow_heap;
pub use hollow_heap::HollowHeap;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
#[cfg(test)]
mod runner;
#[cfg(test)]
mod unit_tests;
