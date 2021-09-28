#[derive(Clone, Debug)]
pub struct Vector<A>
  {
    pub value : Vec<A>
  }

pub fn index<A>(vector : & Vector<A>, index : usize) -> & A
  {
    let & Vector { value : ref vector_value } = vector
  ;
    & vector_value[index]
  }

pub fn index_mutable<A>(vector : & mut Vector<A>, index : usize) -> & mut A
  {
    let & mut Vector { value : ref mut vector_value } = vector
  ;
    & mut vector_value[index]
  }

pub fn get<A>(vector : Vector<& A>, index : usize) -> & A
  {
    self::index(& vector, index).clone()
  }

pub fn set<A>(vector : & mut Vector<A>, index : usize, x : A) -> ()
  {
    crate::pointer::write(index_mutable(vector, index), x)
  }

pub fn pop<A>(vector : & mut Vector<A>) -> Option<A>
  {
    let & mut Vector { value : ref mut vector_value } = vector
  ;
    vector_value.pop()
  }
