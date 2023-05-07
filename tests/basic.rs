use process_owned::{ProcessOwned, ProcessOwnedMut};


#[test]
fn test_basic() {
  let value = ProcessOwned::new(42);
  assert_eq!(*value, 42);
}

#[test]
fn test_clone() {
  let value = ProcessOwned::new(42);
  let value_clone = value.clone();

  assert_eq!(*value, 42);
  assert_eq!(*value_clone, 42);
}

#[test]
fn test_mut() {
  let value = ProcessOwnedMut::new(42);

  let mut value_internal = value.borrow_mut();
  *value_internal = 5;

  assert_eq!(*value_internal, 5);
}

#[test]
fn test_mut_clone() {
  let value = ProcessOwnedMut::new(42);
  let value_clone = value.clone();

  {
    let mut value_internal = value.borrow_mut();
    *value_internal = 5;
  }

  assert_eq!(*value_clone.borrow(), 5);
}