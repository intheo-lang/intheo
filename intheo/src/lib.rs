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
#[derive(Clone, Debug)]
pub struct Address
  {
    value : usize
  }

/// スロットである。
#[derive(Clone, Debug)]
pub enum Slot
  {
    SLOT_1
  ,
    SLOT_2
  ,
    SLOT_3
  }

/// ポートである。
#[derive(Clone, Debug)]
pub struct Port
  {
    pub address : Address
  ,
    pub slot : Slot
  }

/// カインドである。
#[derive(Clone, Debug)]
pub enum Kind
  {
    ERA
  ,
    CON
  ,
    FAN
  }

/// ノードである。
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct Net
  {
    pub nodes : Vec<Node>
  ,
    pub reuse : Vec<Node>
  }

/// `Port` の参照先を求める。
pub fn enter<'a>(net : & 'a Net, port : & 'a Port) -> & 'a Port
  {
    let & Port { address : ref address, slot : ref slot } = port
  ;
    let & ref node = & net.nodes[address.value]
  ;
    let & Node { slot_1 : ref slot_1, slot_2 : ref slot_2, slot_3 : ref slot_3, kind : ref kind } = node
  ;
    match slot
      {
        & Slot::SLOT_1 => slot_1
      ,
        & Slot::SLOT_2 => slot_2
      ,
        & Slot::SLOT_3 => slot_3
      }
  }
