pub trait IdGenerator<T>: Send + Sync {
    fn generate(&self) -> T;
}
