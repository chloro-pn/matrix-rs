use std::fmt::Display;
use std::ops::Index;

pub struct VectorIter<'a, T> {
    index : i64,
    holder : &'a Vector<T>,
}

impl<'a, T> Iterator for VectorIter<'a, T> {
    type Item = &'a T;
    fn next(self : &mut Self) -> Option<Self::Item> {
        if self.index as usize == self.holder.container.len() {
            None
        } else {
            let v = &self.holder.container[self.index as usize];
            self.index += 1;
            Some(v)
        }
    }
}

pub struct Vector<T> {
    container : Vec<T>,
}

impl<T : Clone + Default> Vector<T> {
    pub fn new(length : i64) -> Vector<T> {
        let mut v : Vector<T> = Vector {
            container : Vec::new(),
        };
        v.container.resize(length as usize, Default::default());
        v
    }
}

impl<T : Clone> Vector<T> {
    pub fn new_with(length : i64, value : T) -> Vector<T> {
        let mut v : Vector<T> = Vector {
            container : Vec::new(),
        };
        v.container.resize(length as usize, value);
        v
    }
}

impl<T> Vector<T> {
    pub fn set(self : &mut Self, index : i64, v : T) {
        self.container[index as usize] = v;
    }

    pub fn get_iterator(self : &Self) -> VectorIter<'_, T> {
        VectorIter {
            index : 0,
            holder : self,
        }
    }

    pub fn length(self : &Self) -> usize {
        self.container.len()
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;
    fn index(self : &Self, index : usize) -> &Self::Output {
        &self.container[index]
    }
}

impl<T : Display> Vector<T> {
    pub fn print(self : &Self) {
        println!("v.length() = {}", self.length());
        let mut index : usize = 0;
        for i in self.get_iterator() {
            println!("v[{}] = {}", index, i);
            index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_new() {
        let v : Vector<i32> = Vector::new(3);
        assert_eq!(v.length(), 3);
        let n : i32 = Default::default();
        assert_eq!(v[0], n);
        assert_eq!(v[1], n);
        assert_eq!(v[2], n);
    }

    #[test]
    fn vector_new_with() {
        let v : Vector<i32> = Vector::new_with(3, 11);
        assert_eq!(v.length(), 3);
        let n : i32 = 11;
        assert_eq!(v[0], n);
        assert_eq!(v[1], n);
        assert_eq!(v[2], n);
    }

    #[test]
    #[should_panic]
    fn vector_index_out_of_range() {
        let v : Vector<i32> = Vector::new(3);
        let _ = v[3];
    }

    #[test]
    fn vector_set() {
        let mut v : Vector<i32> = Vector::new(2);
        v.set(0, 12);
        v.set(1, 34);
        assert_eq!(v[0], 12);
        assert_eq!(v[1], 34);
    }

    #[test]
    #[should_panic]
    fn vector_set_out_of_range() {
        let mut v : Vector<i32> = Vector::new(2);
        v.set(2, 11);
    }

    #[test]
    fn vector_iterator() {
        let mut v : Vector<i32> = Vector::new(3);
        v.set(0, 1);
        v.set(1, 2);
        v.set(2, 3);
        let mut count = 1;
        for each in v.get_iterator() {
            assert_eq!(*each, count);
            count += 1;
        }
    }
}