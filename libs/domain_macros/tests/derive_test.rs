use domain_macros::Entity;

// Mock the domain crate structure that the macro expects
mod domain {
    pub trait Entity {
        type Id: PartialEq + Eq;
        fn identity(&self) -> &Self::Id;
    }
}

#[derive(Debug, Entity)]
#[entity(domain_path = "domain")]
struct MyEntity {
    #[entity(id)]
    id: u32,
    #[allow(dead_code)]
    data: String,
}

#[derive(Debug, Entity)]
#[entity(domain_path = "domain")]
enum MyEnum {
    VariantA(MyEntity),
    VariantB(MyEntity),
}

#[test]
fn test_struct_derive() {
    use crate::domain::Entity;
    let e1 = MyEntity {
        id: 1,
        data: "a".into(),
    };
    let e2 = MyEntity {
        id: 1,
        data: "b".into(),
    };
    let e3 = MyEntity {
        id: 2,
        data: "a".into(),
    };

    assert_eq!(e1.identity(), &1);
    assert_eq!(e1, e2);
    assert_ne!(e1, e3);
}

#[test]
fn test_enum_derive() {
    use crate::domain::Entity;
    let e1 = MyEntity {
        id: 1,
        data: "a".into(),
    };
    let e2 = MyEntity {
        id: 2,
        data: "b".into(),
    };

    let v1 = MyEnum::VariantA(e1);
    let v2 = MyEnum::VariantB(MyEntity {
        id: 1,
        data: "c".into(),
    });
    let v3 = MyEnum::VariantA(e2);

    assert_eq!(v1.identity(), &1);
    assert_eq!(v1, v2);
    assert_ne!(v1, v3);
}
