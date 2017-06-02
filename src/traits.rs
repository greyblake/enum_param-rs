pub trait AsEnumParam : Clone {
    type Iter : Iterator + Reset;
    type Item : Clone = <Self::Iter as Iterator>::Item;

    fn iter(&self) -> Self::Iter;
    fn len(&self) -> usize;
    fn rand(&self) -> Self::Item;
}

pub trait Reset {
    fn reset(&mut self);
}
