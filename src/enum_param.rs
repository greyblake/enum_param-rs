use super::*;

#[derive(Clone,Debug)]
pub struct EnumParam<T> {
    items: Vec<T>
}


impl<T> EnumParam<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self { items: items }
    }
}

impl<T: Clone> AsEnumParam for EnumParam<T> {
    type Iter = EnumParamIter<T>;

    fn iter(&self) -> Self::Iter {
        Self::Iter {
            items: self.items.clone(),
            i: 0,
            len: self.items.len()
        }
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}


#[derive(Clone,Debug)]
pub struct EnumParamIter<T> where Vec<T>: Clone {
    items: Vec<T>,
    i: usize,
    len: usize,
}

impl<T: Clone> Iterator for EnumParamIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let cur_i = self.i;
        self.i += 1;

        if cur_i >= self.len {
            None
        } else {
            Some(self.items[cur_i].clone())
        }
    }
}

impl<T: Clone> Reset for EnumParamIter<T>  {
    fn reset(&mut self) {
        self.i = 0;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_param() {
        let param = EnumParam::new(vec![1, 2, 3]);
        let mut iter = param.iter();

        assert_eq!(iter.next().unwrap(), 1);
        assert_eq!(iter.next().unwrap(), 2);
        assert_eq!(iter.next().unwrap(), 3);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        iter.reset();
        assert_eq!(iter.next().unwrap(), 1);
        assert_eq!(iter.next().unwrap(), 2);
        assert_eq!(iter.next().unwrap(), 3);
        assert_eq!(iter.next(), None);
    }

    fn test_len() {
        let param = EnumParam::new(vec![1, 2, 3]);
        assert_eq!(param.len(), 3);

        let param = EnumParam::new(vec!['a', 'b', 'c', 'd']);
        assert_eq!(param.len(), 4);
    }
}
