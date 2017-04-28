pub trait AsEnumParam {
    type Iter : Iterator;

    fn iter(&self) -> Self::Iter;
    fn len(&self) -> usize;
}

pub trait Reset {
    fn reset(&mut self);
}
