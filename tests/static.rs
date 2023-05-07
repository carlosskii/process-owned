use process_owned::Immortal;

#[test]
fn test_static() {
  let immortal_ptr: *const i32;

  unsafe {
    let immortal = Immortal::new(5);
    immortal_ptr = immortal.into();
  }

  assert_eq!(unsafe { *immortal_ptr }, 5);
}