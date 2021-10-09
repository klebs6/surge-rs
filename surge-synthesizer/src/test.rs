#[test] fn test_bincode() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Entity {
        x: f32,
        y: f32,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct World(Vec<Entity>);

    let world = World(vec![Entity { x: 0.0, y: 4.0 }, Entity { x: 10.0, y: 20.5 }]);

    let encoded: Vec<u8> = bincode::serialize(&world).unwrap();

    // 8 bytes for the length of the vector, 4 bytes per float.
    assert_eq!(encoded.len(), 8 + 4 * 4);

    let decoded: World = bincode::deserialize(&encoded[..]).unwrap();

    assert_eq!(world, decoded);
}

#[test] fn test_downcast() {
    use std::any::Any;

    trait A {
        fn as_any(&self) -> &dyn Any;
    }

    struct B;

    impl A for B {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    let a: Box<dyn A> = Box::new(B);
    // The indirection through `as_any` is because using `downcast_ref`
    // on `Box<A>` *directly* only lets us downcast back to `&A` again.
    // The method ensures we get an `Any` vtable that lets us downcast
    // back to the original, concrete type.
    let b: &B = match a.as_any().downcast_ref::<B>() {
        Some(b) => b,
        None => panic!("&a isn't a B!"),
    };
}
