use super::*;

#[derive(Clone,Copy,Debug)]
pub struct EnumParam2<P1, P2> {
    ep1: P1,
    ep2: P2
}

impl<P1, P2> EnumParam2<P1, P2> {
    pub fn new(ep1: P1, ep2: P2) -> Self {
        Self { ep1: ep1, ep2: ep2 }
    }
}

impl<P1, P2> AsEnumParam for EnumParam2<P1, P2>
    where P1: AsEnumParam,
          P2: AsEnumParam,
          <P1 as AsEnumParam>::Iter: Reset,
          <P2 as AsEnumParam>::Iter: Reset,
          <<P1 as AsEnumParam>::Iter as Iterator>::Item: Clone,
          <<P2 as AsEnumParam>::Iter as Iterator>::Item: Clone
    {
    type Iter = EnumParam2Iter<P1::Iter, P2::Iter>;

    fn iter(&self) -> Self::Iter {
        let mut i1 = self.ep1.iter();
        let mut i2 = self.ep2.iter();
        let v1 = i1.next().expect("Iterator i1 returned None");
        let v2 = i2.next().expect("Iterator i2 returned None");
        Self::Iter {
            i1: i1,
            i2: i2,
            v1: v1,
            v2: v2,
            is_finished: false
        }
    }

    fn len(&self) -> usize {
        self.ep1.len() * self.ep2.len()
    }
}


#[derive(Clone,Copy,Debug)]
pub struct EnumParam2Iter<I1: Iterator, I2: Iterator> {
    i1: I1,
    i2: I2,
    v1: I1::Item,
    v2: I2::Item,
    is_finished: bool
}

impl<I1, I2> Reset for EnumParam2Iter<I1, I2>
    where
        I1: Iterator + Reset,
        I2: Iterator + Reset
    {

    fn reset(&mut self) {
        self.i1.reset();
        self.i2.reset();
        let v1 = self.i1.next().expect("Iterator i1 returned None");
        let v2 = self.i2.next().expect("Iterator i2 returned None");
        self.v1 = v1;
        self.v2 = v2;
        self.is_finished = false;
    }
}

impl<I1, I2> Iterator for EnumParam2Iter<I1, I2>
    where I1: Iterator + Reset,
          I2: Iterator + Reset,
          I1::Item : Clone,
          I2::Item : Clone {

    type Item = (I1::Item, I2::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_finished { return None; }

        let out = (self.v1.clone(), self.v2.clone());

        if let Some(new_v2) = self.i2.next() {
            self.v2 = new_v2;
        } else {
            self.i2.reset();
            self.v2 = self.i2.next().expect("i2 returned None!");

            if let Some(new_v1) = self.i1.next() {
                self.v1 = new_v1;
            } else {
                self.is_finished = true;
            }
        }

        Some(out)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_param2() {
        let param1 = EnumParam::new(vec![1.3, 2.4]);
        let param2 = EnumParam::new(vec!['a', 'd', 's']);

        let params = EnumParam2::new(param1, param2);
        let mut iter = params.iter();

        assert_eq!(iter.next().unwrap(), (1.3, 'a'));
        assert_eq!(iter.next().unwrap(), (1.3, 'd'));
        assert_eq!(iter.next().unwrap(), (1.3, 's'));
        assert_eq!(iter.next().unwrap(), (2.4, 'a'));
        assert_eq!(iter.next().unwrap(), (2.4, 'd'));
        assert_eq!(iter.next().unwrap(), (2.4, 's'));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_complex_enum_param2() {
        let a = EnumParam::new(vec![1, 2]);
        let b = EnumParam::new(vec![4.4, 8.8]);
        let c = EnumParam::new(vec!['x', 'y']);

        let ab = EnumParam2::new(a, b);
        let abc = EnumParam2::new(ab, c);

        let mut iter = abc.iter();

        let expected_vals = vec![
            ((1, 4.4), 'x'),
            ((1, 4.4), 'y'),
            ((1, 8.8), 'x'),
            ((1, 8.8), 'y'),
            ((2, 4.4), 'x'),
            ((2, 4.4), 'y'),
            ((2, 8.8), 'x'),
            ((2, 8.8), 'y'),
        ];

        for val in expected_vals.into_iter() {
            assert_eq!(iter.next().unwrap(), val);
        }
    }

    #[test]
    fn test_len() {
        let a = EnumParam::new(vec![1, 2]);
        let b = EnumParam::new(vec![4.4, 8.8]);
        let c = EnumParam::new(vec!['x', 'y', 'z']);

        let ab = EnumParam2::new(a, b);
        assert_eq!(ab.len(), 4);

        let abc = EnumParam2::new(ab, c);
        assert_eq!(abc.len(), 12);
    }
}
