pub struct Port { id : u32 }

pub struct Address { id : u32 }

pub struct Slot { id : u32 }

/// `00` である。
pub const ERA : Slot = Slot { id : 0 };

/// `01` である。
pub const CON : Slot = Slot { id : 1 };

/// `10` である。
pub const FAN : Slot = Slot { id : 2 };

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
