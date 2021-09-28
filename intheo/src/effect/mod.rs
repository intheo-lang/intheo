#[derive(Clone, Debug)]
pub struct Effect<T>
  {
    value : T
  }

impl<T> Effect<T> {
  fn run(self) -> T
    {
      let Effect { value : self_value } = self
    ;
      self_value
    }
}
