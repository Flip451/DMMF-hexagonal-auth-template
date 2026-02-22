use domain::id::IdGenerator;
use uuid::Uuid;

pub struct UuidV7Generator;

impl UuidV7Generator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for UuidV7Generator {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> IdGenerator<T> for UuidV7Generator
where
    T: From<Uuid>,
{
    fn generate(&self) -> T {
        Uuid::now_v7().into()
    }
}
