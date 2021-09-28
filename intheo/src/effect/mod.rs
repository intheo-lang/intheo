/// 作用を伴って生成された値である。
///
///  疑似的に Haskell の IO 型を再現する。ただし、実際には Haskell の IO 型と違って作用の閉じ込めは行われない。そのため、 `Effect` 型の値は `run` メソッドで即時に取り出さなければならない。
#[derive(Clone, Debug)]
pub struct Effect<T>
  {
    pub value : T
  }

impl<T> Effect<T> {
  /// 値を取り出す。
  fn run(self) -> T
    {
      let Effect { value : self_value } = self
    ;
      self_value
    }
}
