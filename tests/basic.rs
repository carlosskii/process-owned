use process_owned::ProcessOwned;


#[test]
fn test_basic() {
  let value = ProcessOwned::new(42);

  assert_eq!(*value, 42);
}

#[test]
fn test_clone() {
  let value = ProcessOwned::new(42);
  let value2 = value.clone();

  assert_eq!(*value, 42);
  assert_eq!(*value2, 42);
}

#[test]
fn test_mut() {
  let mut value = ProcessOwned::new(42);

  *value = 5;

  assert_eq!(*value, 5);
}