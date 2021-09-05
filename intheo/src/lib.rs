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

/// `Address` である。
#[derive(Clone, Debug)]
pub struct Address { pub id : u32 }

/// `Slot` である。値は `ERA` と `CON` と `FAN` の三種類である。
#[derive(Clone, Debug)]
pub struct Slot { pub id : u32 }

/// `00` である。
pub const ERA : Slot = Slot { id : 0 };

/// `01` である。
pub const CON : Slot = Slot { id : 1 };

/// `10` である。
pub const FAN : Slot = Slot { id : 2 };

/// `11` である。
pub const KIND : Slot = Slot { id : 3 };

/// `Port` である。 `Address` と `Slot` のペアである。
#[derive(Clone, Debug)]
pub struct Port { pub id : u32 }

/// `Port` を構築する。
pub fn port(node : Address, slot : Slot) -> Port
  {
    Port { id : (node.id << 2) | slot.id }
  }

/// `Port` から `Address` を取り出す。
pub fn addr(port : Port) -> Address
  {
    Address { id : port.id >> 2 }
  }

/// `Port` から `Slot` を取り出す。
pub fn slot(port : Port) -> Slot
  {
    Slot { id : port.id & 3 }
  }

/// `Net` である。
#[derive(Clone, Debug)]
pub struct Net
  {
    pub nodes: vector::Vector<Port>
  ,
    pub reuse: vector::Vector<Port>
  }

/// もう一方の `Port` を返す。
pub fn enter(net : Net, port : Port) -> Port
  {
    vector::get(& net.nodes, port.id as usize).clone()
  }

/// `Address` の `Slot` を返す。
pub fn kind(net : Net, address : Address) -> Slot
  {
    slot(vector::get(& net.nodes, port(address, KIND).id as usize).clone())
  }

/// 2 つの `Port` を繋ぐ。
pub fn link(net : Net, port_a : Port, port_b : Port) -> Net
  {
    let (port_a_1, port_a_2) = dup(port_a)
  ;
    let (port_b_1, port_b_2) = dup(port_b)
  ;
    Net
      {
        nodes
          :
            vector::set
              (
                vector::set
                  (
                    net.nodes
                  ,
                    port_a_1.id as usize
                  ,
                    port_b_1
                )
              ,
                port_b_2.id as usize
              ,
                port_a_2
              )
      ,
        reuse : net.reuse
      }
  }
