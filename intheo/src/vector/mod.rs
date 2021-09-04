#[derive(Clone, Debug)]
pub struct Vector<A>
  {
    pub value : Vec<A>
  }

pub fn get<A>(vector : & Vector<A>, index : usize) -> & A
  {
    & vector.value[index]
  }

pub fn set<A>(mut vector : Vector<A>, index : usize, x : A) -> Vector<A>
  {
    (& mut vector).value[index] = x
  ;
    vector
  }
