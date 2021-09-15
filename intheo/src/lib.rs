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

/// アドレスである。ノードへのポインタを抽象的に表す。
pub struct Address
  {
    value : usize
  }

/// スロットである。
pub enum Slot
  {
    SLOT_1
  ,
    SLOT_2
  ,
    SLOT_3
  }

/// ポートである。
pub struct Port
  {
    pub address : Address
  ,
    pub slot : Slot
  }

/// カインドである。
pub enum Kind
  {
    ERA
  ,
    CON
  ,
    FAN
  }

/// ノードである。
pub struct Node
  {
    pub slot_1 : Port
  ,
    pub slot_2 : Port
  ,
    pub slot_3 : Port
  ,
    pub kind : Kind
  }

/// ネットである。これはグラフであるが、 Rust では所有権の関係で相互に参照し合うポインタを扱うのが難しいため、単純に配列と添字でグラフを表現している。 ([1]) と ([2]) を参照した。
///
/// [1]: https://qnighy.hatenablog.com/entry/2017/04/28/070000
/// [2]: https://qiita.com/qnighy/items/c3cb525e7f69bee40bf6
pub struct Net
  {
    pub nodes : Vec<Node>
  ,
    pub reuse : Vec<Node>
  }
