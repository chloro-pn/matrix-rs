use std::fmt::Display;
use std::ops::Index;

pub struct VectorIter<'a, T> {
    index : usize,
    holder : &'a Vector<T>,
}

impl<'a, T> Iterator for VectorIter<'a, T> {
    type Item = &'a T;
    fn next(self : &mut Self) -> Option<Self::Item> {
        if self.index == self.holder.container.len() {
            None
        } else {
            let v = &self.holder.container[self.index];
            self.index += 1;
            Some(v)
        }
    }
}

pub struct Vector<T> {
    container : Vec<T>,
}

impl<T : Clone + Default> Vector<T> {
    pub fn new(length : usize) -> Vector<T> {
        let mut v : Vector<T> = Vector {
            container : Vec::new(),
        };
        v.container.resize(length, Default::default());
        v
    }
}

impl<T : Clone> Vector<T> {
    #[warn(dead_code)]
    pub fn new_with(length : usize, value : T) -> Vector<T> {
        let mut v : Vector<T> = Vector {
            container : Vec::new(),
        };
        v.container.resize(length, value);
        v
    }
}

impl<T> Vector<T> {
    pub fn set(self : &mut Self, index : usize, v : T) {
        self.container[index] = v;
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

impl<T : Display> std::fmt::Display for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vector[{}] : \n", self.length()).unwrap();
        let mut index : usize = 0;
        for v in self.get_iterator() {
            write!(f, "[{}] = {} \n", index, v).unwrap();
            index += 1;
        }
        write!(f, "")
    }
}

// 被声明在crate根作用域，并且始终是pub
#[macro_export]
macro_rules! vector {
    ($($var : expr),*) => {{ 
        let mut v = Vec::new();
        $(v.push($var);)*
        let length = v.len();
        let mut vv = Vector::new(length);
        let mut index = 0;
        for each in v.into_iter() {
            vv.set(index, each);
            index += 1;
        }
        vv
    }};
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