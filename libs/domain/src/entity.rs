pub trait Entity {
    type Id: PartialEq + Eq;
    fn identity(&self) -> &Self::Id;
}
