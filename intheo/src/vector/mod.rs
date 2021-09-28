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

pub fn set<A>(mut vector : Vector<A>, index : usize, x : A) -> Vector<A>
  {
    crate::pointer::write(index_mutable(& mut vector, index), x)
  ;
    vector
  }

pub fn pop<A>(mut vector : Vector<A>) -> Option<(A, Vector<A>)>
  {
    match (& mut vector.value).pop()
      {
        Some(vector_p) => Some((vector_p, vector))
      ,
        None => None
      }
  }
