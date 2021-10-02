use crate::effect;

use effect::Effect;

/// 更新する。
pub fn write<T>(pointer : & mut T, value : T) -> Effect<()>
  {
    Effect { value : * pointer = value }
  }
