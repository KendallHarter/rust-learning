use std::fmt::Debug;

pub struct StaticVec<T, const CAPACITY: usize> {
   data: [std::mem::MaybeUninit<T>; CAPACITY],
   size: usize,
}

impl<T, const N: usize> StaticVec<T, N> {
   pub unsafe fn set_len(&mut self, new_size: usize) {
      self.size = new_size;
   }

   pub fn as_slice(&self) -> &[T] {
      &self[..]
   }

   pub fn as_mut_slice(&mut self) -> &mut [T] {
      &mut self[..]
   }

   pub fn swap_remove(&mut self, index: usize) -> T {
      if index >= self.size {
         panic!("blah");
      }
      self.data.swap(index, self.size - 1);
      self.size -= 1;
      // SAFETY: self.data[self.size] is initialized
      unsafe { std::ptr::read(self.data[self.size].as_ptr()) }
   }

   pub fn insert(&mut self, index: usize, element: T) {
      if index > self.size {
         panic!("hoot");
      }
      if self.size >= N {
         panic!("aaaa");
      }
      if index == self.size {
         self.push(element);
      } else {
         // There's probably a better way of doing this?
         // Go from the back, swapping values with the one greater index
         // This creates an uninitialized member at i ready for insertion
         let mut i = self.size;
         while i > index {
            self.data.swap(i, i + 1);
            i -= 1;
         }
         // Swap one last time
         self.data.swap(index, index + 1);
         self.size += 1;
         self.data[index] = std::mem::MaybeUninit::<T>::new(element);
      }
   }

   pub fn remove(&mut self, index: usize) -> T {
      if index >= self.size {
         panic!("aaa");
      }
      for i in index..self.size - 1 {
         self.data.swap(i, i + 1);
      }
      self.size -= 1;
      // SAFETY: self.data[self.size] is initialized
      unsafe { std::ptr::read(self.data[self.size].as_ptr()) }
   }

   pub fn retain<F>(&mut self, mut f: F)
   where
      F: FnMut(&T) -> bool
   {
      for i in (0..self.size).rev() {
         if !f(&self[i]) {
            self.remove(i);
         }
      }
   }

   pub fn retain_mut<F>(&mut self, mut f: F)
   where
      F: FnMut(&mut T) -> bool
   {
      for i in (0..self.size).rev() {
         if !f(&mut self[i]) {
            self.remove(i);
         }
      }
   }

   pub fn new() -> StaticVec<T, N> {
      StaticVec::<T, N> {
         data: [const { std::mem::MaybeUninit::<T>::uninit() }; N],
         size: 0,
      }
   }

   pub fn push(&mut self, value: T) {
      if self.size >= N {
         panic!("doot");
      }
      self.data[self.size] = std::mem::MaybeUninit::<T>::new(value);
      self.size += 1;
   }
}

#[allow(unused_macros)]
macro_rules! count_args {
   () => { 0 };
   ($_first:expr $(, $rest:expr )* $(,)?) => {
      1 + count_args!($($rest),*)
   };
}

#[macro_export]
macro_rules! static_vec {
   ($capacity:expr => $( $values:expr ),*) => {
      {
         const {
            assert!(count_args!($($values),*) <= $capacity);
         }
         #[allow(unused_mut)]
         let mut to_ret = StaticVec::<_, $capacity>::new();
         $(
            to_ret.push($values);
         )*
         to_ret
      }
   };
   ($( $values: expr ),*) => {
      {
         const CAPACITY: usize = count_args!($($values),*);
         static_vec![CAPACITY => $($values),*]
      }
   };
   ($capacity:expr => $value:expr; $size:expr) => {
      {
         const {
            assert!($size <= $capacity);
         }
         #[allow(unused_mut)]
         let mut to_ret = StaticVec::<_, $capacity>::new();
         let val = $value;
         for _ in 0..$size {
            to_ret.push(val);
         }
         to_ret
      }
   };
   ($value:expr; $size:expr) => {
      {
         static_vec![$size => $value; $size]
      }
   };
}

impl<T, const N: usize> std::ops::Deref for StaticVec<T, N> {
   type Target = [T];

   fn deref(&self) -> &Self::Target {
      // SAFETY: 0..self.size is initialized
      //         std::mem::MaybeUninit has same layout as T
      unsafe { std::slice::from_raw_parts(self.data.as_ptr().cast::<T>(), self.size) }
   }
}

impl<T, const N: usize> std::ops::DerefMut for StaticVec<T, N> {
   fn deref_mut(&mut self) -> &mut Self::Target {
      // SAFETY: 0..self.size is initialized
      unsafe { std::slice::from_raw_parts_mut(self.data.as_mut_ptr().cast::<T>(), self.size) }
   }
}

impl<T, const N: usize> std::ops::Drop for StaticVec<T, N> {
   fn drop(&mut self) {
      for v in self.data[0..self.size].iter_mut() {
         // SAFETY: 0..size is initialized
         unsafe {
            v.assume_init_drop();
         }
      }
   }
}

impl<T, const N: usize> std::cmp::PartialEq<[T]> for StaticVec<T, N>
where
   T: PartialEq,
{
   fn eq(&self, other: &[T]) -> bool {
      std::ops::Deref::deref(self) == other
   }
}

impl<T, const N: usize> std::cmp::PartialEq<StaticVec<T, N>> for [T]
where
   T: PartialEq,
{
   fn eq(&self, other: &StaticVec<T, N>) -> bool {
      std::ops::Deref::deref(other) == self
   }
}

impl<T, const CAPACITY: usize, const N: usize> std::cmp::PartialEq<[T; N]> for StaticVec<T, CAPACITY>
where
   T: PartialEq,
{
   fn eq(&self, other: &[T; N]) -> bool {
      std::ops::Deref::deref(self) == other
   }
}

impl<T, const CAPACITY: usize, const N: usize> std::cmp::PartialEq<StaticVec<T, CAPACITY>> for [T; N]
where
   T: PartialEq,
{
   fn eq(&self, other: &StaticVec<T, CAPACITY>) -> bool {
      std::ops::Deref::deref(other) == self
   }
}

impl<T, const N: usize> Debug for StaticVec<T, N>
where
   T: Debug,
{
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_list().entries(self.iter()).finish()
   }
}

impl<T, const N: usize> IntoIterator for StaticVec<T, N> {
   type Item = T;
   type IntoIter = std::iter::Map<
      std::iter::Take<std::array::IntoIter<std::mem::MaybeUninit<T>, N>>,
      fn(std::mem::MaybeUninit<T>) -> T,
   >;

   fn into_iter(mut self) -> Self::IntoIter {
      let old_size = self.size;
      self.size = 0;
      let new_array = std::mem::replace(&mut self.data, [const { std::mem::MaybeUninit::<T>::uninit() }; N]);
      // SAFETY: old_size members are initialized
      new_array
         .into_iter()
         .take(old_size)
         .map(|x| unsafe { std::mem::MaybeUninit::assume_init(x) })
   }
}

impl<'a, T, const N: usize> IntoIterator for &'a StaticVec<T, N> {
   type Item = &'a T;
   type IntoIter = std::iter::Map<
      std::iter::Take<std::slice::Iter<'a, std::mem::MaybeUninit<T>>>,
      fn(&'a std::mem::MaybeUninit<T>) -> &'a T,
   >;

   fn into_iter(self) -> Self::IntoIter {
      // SAFETY: self.size members are initialized
      self
         .data
         .iter()
         .take(self.size)
         .map(|x| unsafe { std::mem::MaybeUninit::assume_init_ref(x) })
   }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut StaticVec<T, N> {
   type Item = &'a mut T;
   type IntoIter = std::iter::Map<
      std::iter::Take<std::slice::IterMut<'a, std::mem::MaybeUninit<T>>>,
      fn(&'a mut std::mem::MaybeUninit<T>) -> &'a mut T,
   >;

   fn into_iter(self) -> Self::IntoIter {
      // SAFETY: self.size members are initialized
      self
         .data
         .iter_mut()
         .take(self.size)
         .map(|x| unsafe { std::mem::MaybeUninit::assume_init_mut(x) })
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn push() {
      let mut v = static_vec![5 =>];
      v.push(1);
      v.push(2);
      v.push(3);
      assert_eq!(v, [1, 2, 3]);
      assert_eq!([1, 2, 3], v);
   }

   #[test]
   fn init() {
      let v = static_vec![5 => 1, 2, 3];
      assert_eq!(v, [1, 2, 3]);
   }

   #[test]
   fn insert() {
      let mut v = static_vec![5 => 1, 2, 3];
      v.insert(0, 0);
      assert_eq!(v, [0, 1, 2, 3]);
      v.insert(4, 4);
      assert_eq!(v, [0, 1, 2, 3, 4]);
   }

   #[test]
   fn remove() {
      let mut v = static_vec![0, 1, 2, 3, 4];
      assert_eq!(v.remove(0), 0);
      assert_eq!(v, [1, 2, 3, 4]);
   }

   #[test]
   fn into_iter() {
      macro_rules! do_loop {
         ($loop_expr:expr, $expected_val:expr) => {
            let mut iters = 0;
            for val in $loop_expr {
               assert_eq!(val, $expected_val);
               iters += 1;
            }
            assert_eq!(iters, 1);
         };
      }

      let mut v = static_vec![1];
      // Test out each kind of into_iter
      do_loop!(&v, &1);
      do_loop!(&mut v, &mut 1);
      do_loop!(v, 1);
   }

   #[test]
   fn retain() {
      let mut v = static_vec![1, 2, 3, 4, 5];
      v.retain(|&x| x % 2 == 0);
      assert_eq!(v, [2, 4]);
   }

   #[test]
   fn retain_mut() {
      let mut v = static_vec![1, 2, 3, 4, 5];
      v.retain_mut(|x| {
         if *x % 2 == 0 {
            *x *= 2;
            true
         }
         else {
            false
         }
      });
   }
}
