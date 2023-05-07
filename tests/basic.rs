use process_owned::ProcessOwned;


#[test]
fn test_basic() {
  let value = ProcessOwned::new(42);

  assert_eq!(*value, 42);
}