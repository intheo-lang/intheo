use crate::effect;

/// 更新する。
pub fn write<T>(pointer : & mut T, value : T) -> effect::Effect<()>
  {
    effect::Effect { value : * pointer = value }
  }
