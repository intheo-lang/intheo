#![allow(non_shorthand_field_patterns)]

pub mod effect;
pub mod pointer;
pub mod vector;

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
    pub nodes : vector::Vector<Node>
  ,
    pub reuse : vector::Vector<Address>
  }

/// `Port` の参照先を取得する。
pub fn enter<'a, 'b>(net : & 'a Net, port : & 'b Port) -> & 'a Port
  {
    let & Net { nodes : ref nodes, reuse : _ } = net
  ;
    let & Port { address : ref address, slot : ref slot } = port
  ;
    let & Address { value : ref address_value } = address
  ;
    let
        Node
          {
            slot_1 : ref slot_1
          ,
            slot_2 : ref slot_2
          ,
            slot_3 : ref slot_3
          ,
            kind : _
          }
      =
        vector::index(nodes, address_value.clone())
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

/// `Address` の参照先のカインドを取得する。
pub fn kind<'a, 'b>(net : & 'a Net, address : & 'b Address) -> & 'a Kind
  {
    let & Net { nodes : ref nodes, reuse : _ } = net
  ;
    let & Address { value : ref address_value } = address
  ;
    let
        Node { slot_1 : _, slot_2 : _, slot_3 : _, kind : ref kind }
      =
        vector::index(nodes, address_value.clone())
  ;
    kind
  }

/// 二つの `Port` を繋ぎ合わせる。
pub fn link(net : Net, port_a : & Port, port_b : & Port) -> Net
  {
    let Net { nodes : mut nodes, reuse : reuse } = net
  ;
    let & Port { address : ref address_a, slot : ref slot_a } = port_a
  ;
    let & Address { value : ref address_value_a } = address_a
  ;
    let & Port { address : ref address_b, slot : ref slot_b } = port_b
  ;
    let & Address { value : ref address_value_b } = address_b
  ;
    let ref_mut_nodes = & mut nodes
  ;
    let
        Node
          {
            slot_1 : ref mut slot_1_a
          ,
            slot_2 : ref mut slot_2_a
          ,
            slot_3 : ref mut slot_3_a
          ,
            kind : _
          }
      =
        vector::index_mutable(ref_mut_nodes, address_value_a.clone())
  ;
    match slot_a
      {
        & Slot::SLOT_1 => pointer::write(slot_1_a, port_b.clone()).run()
      ,
        & Slot::SLOT_2 => pointer::write(slot_2_a, port_b.clone()).run()
      ,
        & Slot::SLOT_3 => pointer::write(slot_3_a, port_b.clone()).run()
      }
  ;
    let
        Node
          {
            slot_1 : ref mut slot_1_b
          ,
            slot_2 : ref mut slot_2_b
          ,
            slot_3 : ref mut slot_3_b
          ,
            kind : _
          }
      =
        vector::index_mutable(ref_mut_nodes, address_value_b.clone())
  ;
    match slot_b
      {
        & Slot::SLOT_1 => pointer::write(slot_1_b, port_a.clone()).run()
      ,
        & Slot::SLOT_2 => pointer::write(slot_2_b, port_a.clone()).run()
      ,
        & Slot::SLOT_3 => pointer::write(slot_3_b, port_a.clone()).run()
      }
  ;
    Net { nodes : nodes, reuse : reuse }
  }

/// `Node` を新しく確保する。
pub fn new_node(net : Net, kind : Kind) -> (Net, Address)
  {
    let Net { nodes : mut nodes, reuse : mut reuse } = net
  ;
    match vector::pop(& mut reuse).run()
      {
          Some(address)
        =>
          {
            let Address { value : address_value } = address
          ;
            let
                node
              =
                Node
                  {
                    slot_1 : Port { address : (& address).clone(), slot : Slot::SLOT_1 }
                  ,
                    slot_2 : Port { address : (& address).clone(), slot : Slot::SLOT_2 }
                  ,
                    slot_3 : Port { address : (& address).clone(), slot : Slot::SLOT_3 }
                  ,
                    kind : kind
                  }
          ;
            vector::set(& mut nodes, address_value, node).run()
          ;
            (Net { nodes : nodes, reuse : reuse }, address)
          }
      ,
          None
        =>
          {
            let address = Address { value : vector::length(& nodes) }
          ;
            let
                node
              =
                Node
                  {
                    slot_1 : Port { address : (& address).clone(), slot : Slot::SLOT_1 }
                  ,
                    slot_2 : Port { address : (& address).clone(), slot : Slot::SLOT_2 }
                  ,
                    slot_3 : Port { address : (& address).clone(), slot : Slot::SLOT_3 }
                  ,
                    kind : kind
                  }
          ;
            vector::push(& mut nodes, node).run()
          ;
            (Net { nodes : nodes, reuse : reuse }, address)
          }
      }
  }
