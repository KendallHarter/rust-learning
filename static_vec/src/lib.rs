use std::fmt::Debug;

pub struct StaticVec<T, const CAPACITY: usize> {
   data: [std::mem::MaybeUninit<T>; CAPACITY],
   size: usize,
}

impl<T, const N: usize> StaticVec<T, N> {
   pub unsafe fn set_len(&mut self, new_size: usize) {
      self.size = new_size;
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

   pub fn new() -> StaticVec<T, N> {
      StaticVec::<T, N> {
         data: [const { std::mem::MaybeUninit::<T>::uninit() }; N],
         size: 0,
      }
   }

   pub fn push(&mut self, value: T) {
      if self.size == N {
         panic!("doot");
      }
      self.data[self.size] = std::mem::MaybeUninit::<T>::new(value);
      self.size += 1;
   }
}

impl<T, const N: usize> std::ops::Deref for StaticVec<T, N> {
   type Target = [T];

   fn deref(&self) -> &Self::Target {
      // SAFETY: 0..self.size is initialized
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
      let mut v = StaticVec::<i32, 10>::new();
      v.push(1);
      v.push(2);
      v.push(3);
      assert_eq!(v, [1, 2, 3]);
      assert_eq!([1, 2, 3], v);
   }

   #[test]
   fn into_iter() {
      let mut v = StaticVec::<i32, 10>::new();
      v.push(1);
      for val in v {
         assert_eq!(val, 1);
      }
   }
}
