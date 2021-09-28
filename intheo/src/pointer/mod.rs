/// 更新する。
pub fn write<T>(pointer : & mut T, value : T) -> ()
  {
    * pointer = value
  }
