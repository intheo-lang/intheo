/// 更新する。
pub fn write<T>(pointer : & mut T, value : T) -> crate::effect::Effect<()>
  {
    crate::effect::Effect { value : * pointer = value }
  }
