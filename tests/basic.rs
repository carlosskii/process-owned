use process_owned::ProcessOwned;


#[test]
fn test_basic() {
  let value = ProcessOwned::new(42);

  let value_internal = value.borrow();
  assert_eq!(*value_internal, 42);
}

#[test]
fn test_clone() {
  let value = ProcessOwned::new(42);
  let value_clone = value.clone();

  let value_internal = value.borrow();
  assert_eq!(*value_internal, 42);

  let value_clone_internal = value_clone.borrow();
  assert_eq!(*value_clone_internal, 42);
}

#[test]
fn test_mut() {
  let value = ProcessOwned::new(42);

  let mut value_internal = value.borrow_mut();
  *value_internal = 5;

  assert_eq!(*value_internal, 5);
}