pub mod vector;

/// 複製する。
pub fn dup<T>(value : T) -> (T, T) where T : Clone
  {
    let ref_value : &T = &value
  ;
    let new_value_1 : T = ref_value.clone()
  ;
    let new_value_2 : T = ref_value.clone()
  ;
    (new_value_1, new_value_2)
  }
