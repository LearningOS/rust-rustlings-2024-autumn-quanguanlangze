struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    let mut ret: Box<Foo> = Box::from_raw(ptr); // SAFETY: Construct the box from the raw pointer
    ret.b = Some("hello".to_owned()); // Modify the box content safely
    ret // Return the modified box
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize;
        // SAFETY: We pass an owned box of `Foo`.
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = &ret.a as *const u128 as usize;

        assert!(ptr_1 == ptr_2); // Ensure that the pointer addresses match
        assert!(ret.b == Some("hello".to_owned())); // Ensure that the content is correctly updated
    }
}
