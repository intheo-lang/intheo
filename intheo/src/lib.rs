#![allow(non_shorthand_field_patterns)]
#![allow(non_camel_case_types)]

pub mod effect;
pub mod pointer;
pub mod vector;

use effect::Effect;

/// アドレスである。ノードへのポインタを抽象的に表す。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Address
  {
    value : usize
  }

/// スロットである。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Slot
  {
    SLOT_0
  ,
    SLOT_1
  ,
    SLOT_2
  }

/// ポートである。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Port
  {
    pub address : Address
  ,
    pub slot : Slot
  }

/// カインドである。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Kind
  {
    ERA
  ,
    CON
  ,
    FAN
  }

/// ノードである。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Node
  {
    pub slot_0 : Port
  ,
    pub slot_1 : Port
  ,
    pub slot_2 : Port
  ,
    pub kind : Kind
  }

/// ネットである。これはグラフであるが、 Rust では所有権の関係で相互に参照し合うポインタを扱うのが難しいため、単純に配列と添字でグラフを表現している。 ([1]) と ([2]) を参照した。
///
/// [1]: https://qnighy.hatenablog.com/entry/2017/04/28/070000
/// [2]: https://qiita.com/qnighy/items/c3cb525e7f69bee40bf6
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Net
  {
    pub nodes : vector::Vector<Node>
  ,
    pub reuse : vector::Vector<Address>
  }

/// `Port` の参照先を取得する。
pub fn enter(net : & Net, port : Port) -> & Port
  {
    let & Net { nodes : ref nodes, reuse : _ } = net
  ;
    let Port { address : address, slot : slot } = port
  ;
    let Address { value : address_value } = address
  ;
    let
        &
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
        vector::index(nodes, address_value)
  ;
    match slot
      {
        Slot::SLOT_0 => slot_0
      ,
        Slot::SLOT_1 => slot_1
      ,
        Slot::SLOT_2 => slot_2
      }
  }

/// `Address` の参照先のカインドを取得する。
pub fn kind(net : & Net, address : Address) -> & Kind
  {
    let & Net { nodes : ref nodes, reuse : _ } = net
  ;
    let Address { value : address_value } = address
  ;
    let
        Node { slot_0 : _, slot_1 : _, slot_2 : _, kind : ref kind }
      =
        vector::index(nodes, address_value)
  ;
    kind
  }

/// 二つの `Port` を繋ぎ合わせる。
pub fn link(net : & mut Net, port_a : & Port, port_b : & Port) -> Effect<()>
  {
    let & mut Net { nodes : ref mut nodes, reuse : _ } = net
  ;
    let & Port { address : ref address_a, slot : ref slot_a } = port_a
  ;
    let & Address { value : ref address_value_a } = address_a
  ;
    let & Port { address : ref address_b, slot : ref slot_b } = port_b
  ;
    let & Address { value : ref address_value_b } = address_b
  ;
    {
      let
          & mut
            Node
              {
                slot_0 : ref mut slot_0_a
              ,
                slot_1 : ref mut slot_1_a
              ,
                slot_2 : ref mut slot_2_a
              ,
                kind : _
              }
        =
          vector::index_mutable(nodes, address_value_a.clone())
    ;
      match slot_a
        {
          & Slot::SLOT_0 => pointer::write(slot_0_a, port_b.clone()).run()
        ,
          & Slot::SLOT_1 => pointer::write(slot_1_a, port_b.clone()).run()
        ,
          & Slot::SLOT_2 => pointer::write(slot_2_a, port_b.clone()).run()
        }
    }
  ;
    {
      let
          & mut
            Node
              {
                slot_0 : ref mut slot_0_b
              ,
                slot_1 : ref mut slot_1_b
              ,
                slot_2 : ref mut slot_2_b
              ,
                kind : _
              }
        =
          vector::index_mutable(nodes, address_value_b.clone())
    ;
      match slot_b
        {
          & Slot::SLOT_0 => pointer::write(slot_0_b, port_a.clone()).run()
        ,
          & Slot::SLOT_1 => pointer::write(slot_1_b, port_a.clone()).run()
        ,
          & Slot::SLOT_2 => pointer::write(slot_2_b, port_a.clone()).run()
        }
    }
  ;
    Effect { value : () }
  }

/// `Node` を新しく確保する。
pub fn new_node(net : & mut Net, kind : & Kind) -> Effect<Address>
  {
    let & mut Net { nodes : ref mut nodes, reuse : ref mut reuse } = net
  ;
    let option_address = vector::pop(reuse).run()
  ;
    match option_address
      {
          Some(address)
        =>
          {
            let
                node
              =
                Node
                  {
                    slot_0 : Port { address : (& address).clone(), slot : Slot::SLOT_0 }
                  ,
                    slot_1 : Port { address : (& address).clone(), slot : Slot::SLOT_1 }
                  ,
                    slot_2 : Port { address : (& address).clone(), slot : Slot::SLOT_2 }
                  ,
                    kind : kind.clone()
                  }
          ;
            {
              let Address { value : address_value } = address
            ;
              vector::set(nodes, address_value, node).run()
            }
          ;
            Effect { value : address }
          }
      ,
          None
        =>
          {
            let
                address
              =
                Address
                  {
                      value
                    :
                      {
                        let & mut ref nodes_immutable = nodes
                      ;
                        vector::length(nodes_immutable)
                      }
                  }
          ;
            let
                node
              =
                Node
                  {
                    slot_0 : Port { address : (& address).clone(), slot : Slot::SLOT_0 }
                  ,
                    slot_1 : Port { address : (& address).clone(), slot : Slot::SLOT_1 }
                  ,
                    slot_2 : Port { address : (& address).clone(), slot : Slot::SLOT_2 }
                  ,
                    kind : kind.clone()
                  }
          ;
            vector::push(nodes, node).run()
          ;
            Effect { value : address }
          }
      }
  }

/// `reduce` の統計である。
pub struct Statics
  {
    pub loops : u32
  ,
    pub rules : u32
  }

/// `Address` で指定された `Node` のペアを書き換える。
pub fn rewrite(net : & mut Net, x : & Address, y : & Address) -> Effect<()>
  {
    if
      {
        let & mut ref net_immutable = net
      ;
        kind(net_immutable, x).clone() == kind(net_immutable, y).clone()
      }
      {
        {
          let
              x_1
            =
              {
                let & mut ref net_immutable = net
              ;
                enter(net_immutable, & Port { address : x.clone(), slot : Slot::SLOT_1 }).clone()
              }
        ;
          let
              y_1
            =
              {
                let & mut ref net_immutable = net
              ;
                enter(net_immutable, & Port { address : y.clone(), slot : Slot::SLOT_1 }).clone()
              }
        ;
          link(net, & x_1, & y_1).run()
        }
      ;
        {
          let
              x_2
            =
              {
                let & mut ref net_immutable = net
              ;
                enter(net_immutable, & Port { address : x.clone(), slot : Slot::SLOT_2 }).clone()
              }
        ;
          let
              y_2
            =
              {
                let & mut ref net_immutable = net
              ;
                enter(net_immutable, & Port { address : y.clone(), slot : Slot::SLOT_2 }).clone()
              }
        ;
          link(net, & x_2, & y_2).run()
        }
      ;
        let & mut Net { nodes : _, reuse : ref mut reuse } = net
      ;
        vector::push(reuse, x.clone()).run()
      ;
        vector::push(reuse, y.clone()).run()
      ;
        Effect { value : () }
      }
    else
      {
        Effect { value : () }
      }
  }

/// 簡約する。
pub fn reduce(net : & mut Net, statics : & mut Statics) -> Effect<()>
  {
    Effect { value : () }
  }
