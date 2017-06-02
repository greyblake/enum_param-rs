use super::*;

#[derive(Clone,Copy,Debug)]
pub struct EnumParam3<P1, P2, P3> {
    ep1: P1,
    ep2: P2,
    ep3: P3
}

impl<P1, P2, P3> EnumParam3<P1, P2, P3> {
    pub fn new(ep1: P1, ep2: P2, ep3: P3) -> Self {
        Self { ep1: ep1, ep2: ep2, ep3: ep3 }
    }
}

impl<P1, P2, P3> AsEnumParam for EnumParam3<P1, P2, P3>
    where P1: AsEnumParam,
          P2: AsEnumParam,
          P3: AsEnumParam,
          <P1 as AsEnumParam>::Iter: Reset,
          <P2 as AsEnumParam>::Iter: Reset,
          <P3 as AsEnumParam>::Iter: Reset,
          <<P1 as AsEnumParam>::Iter as Iterator>::Item: Clone,
          <<P2 as AsEnumParam>::Iter as Iterator>::Item: Clone,
          <<P3 as AsEnumParam>::Iter as Iterator>::Item: Clone
    {
    type Iter = EnumParam3Iter<P1::Iter, P2::Iter, P3::Iter>;
    type Item = (<P1 as AsEnumParam>::Item, <P2 as AsEnumParam>::Item, <P3 as AsEnumParam>::Item);

    fn iter(&self) -> Self::Iter {
        let mut i1 = self.ep1.iter();
        let mut i2 = self.ep2.iter();
        let mut i3 = self.ep3.iter();
        let v1 = i1.next().expect("Iterator i1 returned None");
        let v2 = i2.next().expect("Iterator i2 returned None");
        let v3 = i3.next().expect("Iterator i3 returned None");
        Self::Iter {
            i1: i1,
            i2: i2,
            i3: i3,
            v1: v1,
            v2: v2,
            v3: v3,
            is_finished: false
        }
    }

    fn len(&self) -> usize {
        self.ep1.len() * self.ep2.len() * self.ep3.len()
    }

    fn rand(&self) -> Self::Item {
        (self.ep1.rand(), self.ep2.rand(), self.ep3.rand())
    }
}


#[derive(Clone,Copy,Debug)]
pub struct EnumParam3Iter<I1: Iterator, I2: Iterator, I3: Iterator> {
    i1: I1,
    i2: I2,
    i3: I3,
    v1: I1::Item,
    v2: I2::Item,
    v3: I3::Item,
    is_finished: bool
}

impl<I1, I2, I3> Reset for EnumParam3Iter<I1, I2, I3>
    where
        I1: Iterator + Reset,
        I2: Iterator + Reset,
        I3: Iterator + Reset
    {

    fn reset(&mut self) {
        self.i1.reset();
        self.i2.reset();
        self.i3.reset();
        self.v1 = self.i1.next().expect("Iterator i1 returned None");
        self.v2 = self.i2.next().expect("Iterator i2 returned None");
        self.v3 = self.i3.next().expect("Iterator i3 returned None");
        self.is_finished = false;
    }
}

impl<I1, I2, I3> Iterator for EnumParam3Iter<I1, I2, I3>
    where I1: Iterator + Reset,
          I2: Iterator + Reset,
          I3: Iterator + Reset,
          I1::Item : Clone,
          I2::Item : Clone,
          I3::Item : Clone {

    type Item = (I1::Item, I2::Item, I3::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_finished { return None; }

        let out = (self.v1.clone(), self.v2.clone(), self.v3.clone());

        if let Some(new_v3) = self.i3.next() {
            self.v3 = new_v3;
        } else {
            self.i3.reset();
            self.v3 = self.i3.next().expect("i3 returned None!");

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
        }

        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_param3() {
        let p1 = EnumParam::new(vec![0, 1]);
        let p2 = p1.clone();
        let p3 = p1.clone();

        let params = EnumParam3::new(p1, p2, p3);
        let mut iter = params.iter();
        assert_eq!(iter.next().unwrap(), (0, 0, 0));
        assert_eq!(iter.next().unwrap(), (0, 0, 1));
        assert_eq!(iter.next().unwrap(), (0, 1, 0));
        assert_eq!(iter.next().unwrap(), (0, 1, 1));
        assert_eq!(iter.next().unwrap(), (1, 0, 0));
        assert_eq!(iter.next().unwrap(), (1, 0, 1));
        assert_eq!(iter.next().unwrap(), (1, 1, 0));
        assert_eq!(iter.next().unwrap(), (1, 1, 1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_len() {
        let a = EnumParam::new(vec![1, 2]);
        let b = EnumParam::new(vec![4.4, 8.8, 9.0]);
        let c = EnumParam::new(vec!['x', 'y', 'z']);

        let abc = EnumParam3::new(a, b, c);
        assert_eq!(abc.len(), 18);
    }

    #[test]
    fn test_rand() {
        let a = EnumParam::new(vec![1, 2, 3, 4]);
        let b = EnumParam::new(vec![4.4, 8.8, 9.0]);
        let c = EnumParam::new(vec!['x', 'y', 'z']);
        let abc = EnumParam3::new(a, b, c);

        println!("{:?}", abc.rand());
    }
}
