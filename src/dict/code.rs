use crate::dict::storage::*;
use crate::dict::*;
#[derive(Clone, Debug)]
pub enum Code{
    Object(Object),
    Keyword(KwID),
}
impl PartialEq for Code{
    fn eq(&self, other:&Code) -> bool{
        match self{
            Code::Keyword(id) => {if let Code::Keyword(id2) = other{id == id2} else {false}},
            Code::Object(id) => {if let Code::Object(id2) = other{id == id2} else {false}},
        }
    }
}
#[derive(Clone, Debug)]
pub enum Object{
    Variable(String),
    Literal(Data),
    Listeral(Vec<Vec<Code>>),
    Parens(Vec<Vec<Code>>),
}
impl PartialEq for Object{
    fn eq(&self, other:&Object) -> bool{
        match self{
            Object::Variable(_) =>{if let Object::Variable(_) = other{true}else{false}} 
            Object::Literal(val1) =>{if let Object::Literal(val2) = other{val1 == val2}else{false}}
            Object::Listeral(_) =>{if let Object::Listeral(_) = other{true}else{false}}
            Object::Parens(_) =>{if let Object::Parens(_) = other{true}else{false}}
        }
    }
}
impl PartialEq for Data{
    fn eq(&self, other:&Data) -> bool{
        match self{
            Data::Abstr(_,_) =>{if let Data::Abstr(_,_) = other{true}else{false}},
            Data::Func(_) =>{if let Data::Func(_) = other{true}else{false}},
            Data::Type(_) => {if let Data::Type(_) = other{true}else{false}}
            _=>match other{
                Data::Abstr(_,_)|
                Data::Func(_)|
                Data::Type(_) => false,
                _=>true,
            }
        }
    }
}
impl Code{
    pub fn priority(&self, dict:&mut Dict) -> usize{
        match self{
            Code::Keyword(id)=>dict.get_kw(*id).priority,
            Code::Object(id)=>id.priority(),
        }
    }
    pub fn simplify(&self) -> Code{
        match self{
            Code::Keyword(_)=>self.clone(),
            Code::Object(id)=>id.simplify(),
        }
    }
    pub fn search(&self, potential:&Vec<Code>) -> Option<usize>{
        for i in 0..potential.len(){
            if self == (&potential[i]){
                return Some(i);
            }
        }
        return None;
    }
}
impl Object{
    pub fn simplify(&self) -> Code{
        match self{
            Object::Listeral(_)=>Code::Object(Object::Listeral(vec![])),
            Object::Literal(val)=>Code::Object(Object::Literal(val.clone().simplify())),
            Object::Parens(_)=>Code::Object(Object::Parens(vec![])),
            Object::Variable(_)=>Code::Object(Object::Variable("".to_string())),
        }
    }
}
impl Data{
    pub fn simplify(self) -> Data{
        match self{
            Data::Abstr(_,x)=>Data::Abstr(vec![], x),
            Data::Func(_)=>Data::Func(FnPntr::new(0)),
            _=>self
        }
    }
}
