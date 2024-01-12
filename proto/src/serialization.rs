


pub(crate) trait BoltValue : Sized {
    fn marker(&self);
    fn serialize(self);
}


pub(crate) trait BoltStructure {
    fn signature(&self) -> u8;
}