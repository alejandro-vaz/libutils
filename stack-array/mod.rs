//^
//^ HEAD
//^

//> HEAD -> NO_STD
#![no_std]

//> HEAD -> DOCS
#![doc = include_str!("README.md")]

//> HEAD -> LINTS
#![allow(incomplete_features)]

//> HEAD -> FEATURES
#![feature(const_cmp)]
#![feature(const_destruct)]
#![feature(const_drop_in_place)]
#![feature(const_array)]
#![feature(transmute_neo)]
#![feature(const_index)]
#![feature(const_range)]
#![feature(maybe_uninit_uninit_array_transpose)]
#![feature(const_closures)]
#![feature(const_trait_impl)]
#![feature(box_vec_non_null)]
#![feature(trusted_len)]
#![feature(const_clone)]
#![feature(new_range)]
#![feature(const_slice_make_iter)]
#![feature(const_ops)]
#![feature(generic_const_exprs)]
#![feature(const_iter)]
#![feature(const_convert)]
#![feature(const_default)]

//> HEAD -> CRATES
extern crate alloc;

//> HEAD -> MODULES
mod comparisons;
mod conversions;
mod iterators;
mod references;

//> HEAD -> CORE
use core::{
    fmt::{
        Debug,
        Formatter,
        Result as Format
    }, 
    marker::Destruct, 
    mem::{
        MaybeUninit,
        forget,
        transmute_neo as transmute,
        replace
    }, 
    ops::{
        Bound, 
        Drop, 
        RangeBounds, 
        SubAssign
    }, 
    array::from_fn as arrayfn,
    ptr::copy
};

//> HEAD -> CONSTRANGEITER
use constrangeiter::ConstIntoIterator;


//^
//^ ARRAY
//^

//> ARRAY -> STRUCT
pub struct Array<Type, const N: usize> {
    length: usize,
    data: [MaybeUninit<Type>; N]
}

//> ARRAY -> IMPLEMENTATION
impl<Type, const N: usize> Array<Type, N> {
    pub const fn len(&self) -> usize {return self.length}
    pub const fn new() -> Self {return Self::default()}
    pub const fn is_full(&self) -> bool {return self.length == N}
    pub const fn repeat<const TIMES: usize>(self) -> Array<Type, {TIMES * N}> where Type: [const] Clone + [const] Destruct, [(); TIMES * N]: {
        let (length, mut data) = self.into();
        let mut additional = MaybeUninit::<[Type; TIMES * N]>::uninit().transpose();
        if TIMES == 0 {for index in (0..length).const_into_iter() {
            unsafe {data[index].assume_init_drop();};
        }} else {
            for index in (0..length).const_into_iter() {
                additional[index].write(unsafe {data[index].assume_init_read()});
            }
            for iteration in (1..TIMES).const_into_iter() {
                for index in (0..length).const_into_iter() {
                    additional[index + length * iteration].write(unsafe {
                        data[index].assume_init_ref().clone()
                    });
                }
            }
        }
        return Array::from((length * TIMES, additional));
    }
    pub const fn resize<const M: usize>(self) -> Array<Type, M> where Type: [const] Destruct {
        let (length, mut data) = self.into();
        let mut additional = MaybeUninit::<[Type; M]>::uninit().transpose();
        return if M >= length {
            for index in (0..length).const_into_iter() {
                additional[index].write(unsafe {data[index].assume_init_read()});
            }
            Array::from((length, additional))
        } else {
            for index in (0..M).const_into_iter() {
                additional[index].write(unsafe {data[index].assume_init_read()});
            }
            for index in (M..length).const_into_iter() {
                unsafe {data[index].assume_init_drop()};
            }
            Array::from((M, additional))
        }
    }
    pub const fn divide<const AT: usize>(self) -> (
        Array<Type, AT>, 
        Array<Type, {N - AT}>
    ) where [(); N - AT]: {
        let (length, data) = self.into();
        let (first, second) = unsafe {transmute(data)};
        return (Array {
            length: length.min(AT),
            data: first
        }, Array {
            length: length.saturating_sub(AT),
            data: second
        })
    }
    pub const fn push(&mut self, value: Type) -> () {
        self.push_mut(value);
    }
    pub const fn push_mut<'valid>(&'valid mut self, value: Type) -> &'valid mut Type {
        let reference = self.data[self.length].write(value);
        self.length += 1;
        return reference;
    }
    pub const fn pop(&mut self) -> Option<Type> {return if self.length == 0 {None} else {
        self.length -= 1;
        Some(unsafe {self.data[self.length].assume_init_read()})
    }}
    pub const fn pop_if(
        &mut self, 
        function: impl [const] FnOnce(&mut Type) -> bool + [const] Destruct
    ) -> Option<Type> {return if self.length == 0 {None} else {
        let last = &mut self.data[self.length - 1];
        if function(unsafe {last.assume_init_mut()}) {
            self.length -= 1;
            Some(unsafe {replace(
                last,
                MaybeUninit::uninit()
            ).assume_init()})
        } else {None}
    }}
    pub const fn clear(&mut self) -> () where Type: [const] Destruct {self.truncate(0)}
    pub const fn truncate(&mut self, length: usize) -> () where Type: [const] Destruct {
        for index in (length..self.length).const_into_iter() {
            unsafe {self.data.get_unchecked_mut(index).assume_init_drop()};
        }
        self.length = length;
    }
    pub const fn insert(&mut self, index: usize, value: Type) -> () {
        self.insert_mut(index, value);
    }
    pub const fn insert_mut<'valid>(
        &'valid mut self, 
        index: usize, 
        value: Type
    ) -> &'valid mut Type {
        assert!(index <= self.length, "tried to insert out of bounds");
        assert!(self.length != N, "array capacity exceeded");
        let pointer = unsafe {self.data.as_mut_ptr().add(index)};
        unsafe {copy(
            pointer,
            pointer.add(1),
            self.length - index
        )};
        let reference = unsafe {self.data.get_unchecked_mut(index).write(value)};
        self.length += 1;
        return reference;
    }
    pub const fn remove(&mut self, index: usize) -> Type {
        assert!(index < self.length, "tried to remove out of bounds");
        let value = unsafe {self.data.get_unchecked(index).assume_init_read()};
        let pointer = unsafe {self.data.as_mut_ptr().add(index)};
        unsafe {copy(
            pointer.add(1),
            pointer,
            self.length - index - 1
        )};
        self.length -= 1;
        return value;
    }
    pub const fn swap_remove(&mut self, index: usize) -> Type {
        assert!(index < self.length - 1, "tried to remove out of bounds");
        let value = unsafe {self.data[index].assume_init_read()};
        self.data.swap(index, self.length - 1);
        self.length -= 1;
        return value;
    }
    pub const fn retain(
        &mut self, 
        mut closure: impl [const] FnMut(&mut Type) -> bool + [const] Destruct
    ) -> () where Type: [const] Destruct {
        let mut offset = 0;
        for index in (0..self.length).const_into_iter() {
            let mut item = unsafe {self.data[index].assume_init_read()};
            match (closure(&mut item), offset == 0) {
                (true, true) => forget(item),
                (true, false) => {self.data[index - offset].write(item);},
                (false, _) => {
                    drop(item);
                    offset += 1;
                }
            }
        }
        self.length -= offset;
    }
    pub const fn dedup(&mut self) -> () where Type: [const] PartialEq<Type> + [const] Destruct {
        self.dedup_by(const |first, second| (first as &Type).eq(second as &Type));
    }
    pub const fn dedup_by(
        &mut self, 
        mut decider: impl [const] FnMut(&mut Type, &mut Type) -> bool + [const] Destruct
    ) -> () where Type: [const] Destruct {
        if self.length < 2 {return}
        let mut offset = 0;
        let mut first = None;
        for index in (0..self.length).const_into_iter() {
            let mut second = unsafe {self.data[index].assume_init_read()};
            if first.is_none() {
                first = Some(second);
                continue;
            }
            if decider(first.as_mut().unwrap(), &mut second) {
                drop(second);
                offset += 1;
            } else {
                if offset == 0 {
                    forget(first.replace(second).unwrap());
                } else {
                    let pointer = self.data[index - offset].write(second) as *mut Type;
                    forget(first.replace(
                        unsafe {pointer.read()}
                    ).unwrap());
                }
            }
        }
        self.length.sub_assign(offset);
    }
    pub const fn dedup_by_key<Key: [const] PartialEq<Key> + [const] Destruct>(
        &mut self,
        mut transformation: impl [const] FnMut(&mut Type) -> Key + [const] Destruct
    ) -> () where Type: [const] Destruct {
        self.dedup_by(const |first, second| transformation(first) == transformation(second));
    }
    pub const fn drain(
        &mut self, 
        range: impl [const] RangeBounds<usize> + [const] Destruct
    ) -> Self {
        let start = match range.start_bound() {
            Bound::Excluded(_) => unreachable!(),
            Bound::Included(bound) => { // 2.. => 2
                assert!(*bound < self.length);
                *bound
            },
            Bound::Unbounded => 0
        };
        let end = match range.end_bound() {
            Bound::Excluded(bound) => { // ..2 => 2
                assert!(*bound <= self.length);
                *bound
            },
            Bound::Included(bound) => { // ..=2 => 3
                assert!(*bound < self.length);
                *bound + 1
            },
            Bound::Unbounded => self.length
        };
        let mut additional = MaybeUninit::<[Type; N]>::uninit().transpose();
        let array = match end - start {
            0 => Array {
                length: 0,
                data: additional
            },
            1 => {
                additional[0].write(self.remove(start));
                Array {
                    length: 1,
                    data: additional
                }
            },
            amount => {
                for index in (start..end).const_into_iter() {
                    additional[index - start].write(unsafe {
                        self.data[index].assume_init_read()
                    });
                }
                for index in (end..self.length).const_into_iter() {
                    self.data[index - end + start].write(unsafe {
                        self.data[index].assume_init_read()
                    });
                }
                self.length -= end - start;
                Array {
                    length: amount,
                    data: additional
                }
            }
        };
        return array;
    }
}

//> ARRAY -> DROP
const impl<Type: [const] Destruct, const N: usize> Drop for Array<Type, N> {
    fn drop(&mut self) {self.clear()}
}

//> ARRAY -> DEBUG
impl<Type: Debug, const N: usize> Debug for Array<Type, N> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Format {
        return Debug::fmt(self.as_ref(), formatter);
    }
}

//> ARRAY -> EXTEND
impl<Type, const N: usize> Extend<Type> for Array<Type, N> {
    fn extend<T: IntoIterator<Item = Type>>(&mut self, iter: T) {
        iter.into_iter().for_each(|item| self.push(item));
    }
}

//> ARRAY -> CLONE
const impl<Type: [const] Clone, const N: usize> Clone for Array<Type, N> {
    fn clone(&self) -> Self {return Array {
        length: self.length,
        data: arrayfn(const |index| if index >= self.length {MaybeUninit::uninit()} else {
            MaybeUninit::new(unsafe {self.data[index].assume_init_ref().clone()})
        })
    }}
}

//> ARRAY -> DEFAULT
const impl<Type, const N: usize> Default for Array<Type, N> {
    fn default() -> Self {return Self {
        data: MaybeUninit::uninit().transpose(),
        length: 0
    }}
}

pub fn test(x: u8, mut array: Array<u8, 4>) -> () {
    array.push(x);
}