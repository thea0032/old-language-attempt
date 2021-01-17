use crate::dict::subdict::*;
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type{
    Bool,
    Int,
    Unsigned,
    Double,
    Num,
    Char,
    List(Box<Type>),
    Func,
    Abstr,
    Tup(Vec<Type>, usize),
    Null,
    Any,
    Type,
    Subdict(SubID),
    Ref,
    File,
}
pub enum Next{
    None,
    Some(usize),
}
impl Type{
    pub fn get_next(&self) -> Next{
        match self{
            Type::List(_)=>Next::Some(1),
            Type::Tup(_, val)=>Next::Some(*val),
            _=>Next::None,
        }
    }
    pub fn add_to_field(&mut self, pos:usize, typ:Type){
        match self{
            Type::List(this)=>{if pos == 0{**this = typ}else{panic!("Operation add_to failed!");}},
            Type::Tup(data, size)=>{if pos < *size{data[pos] = typ;}else{panic!("Operation add_to failed!");}}
            _=>{panic!("Type {:?} has no fields!", self)}
        }
    }
    pub fn field_is_null(&self, pos:usize) -> bool{
        match self{
            Type::List(this)=>**this == Type::Null,
            Type::Tup(data, size)=>{if pos < *size{data[pos] == Type::Null}else{panic!("Operation add_to failed!");}}
            _=>{panic!("Type {:?} has no fields!", self)}
        }
    }
}