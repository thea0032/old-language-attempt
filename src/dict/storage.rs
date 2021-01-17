
use crate::dict::code::*;
use crate::dict::*;
use crate::dict::typ::Type;
use crate::dict::subdict::*;
#[derive(Clone, Debug)]
pub struct Line{
    pub users:usize,
    pub data:Vec<Data>,
    pub line:usize,
}
impl Line{
    pub fn new(line:usize) -> Line{
        Line{users:0, data:vec![], line:line}
    }
}
#[derive(Clone, Debug)]
pub struct Block{
    pub users:usize,
    pub data:Data,
    pub line:usize,
}
impl Block{
    #[allow(dead_code)]
    pub fn new(line:usize) -> Block{
        Block{users:0, data:Data::Null, line:line}
    }
}
#[derive(Clone, Debug)]
pub struct FnBlock{
    pub users:usize,
    pub data:Func,
    pub line:usize,
}
impl FnBlock{
    pub fn new(line:usize, init:Func) -> FnBlock{
        FnBlock{users:0, data:init, line:line}
    }
    pub fn alter(&mut self, line:usize, new:Func){
        self.data = new;
        self.users = 0;
        self.line = line;
    }
}
#[derive(Clone, Debug, Copy)]
pub enum Pntr{
    Line(LinePntr),
    Location(LocationPntr),
    Stack(StackPntr),
    Func(FnPntr),
}
#[derive(Clone, Debug, Copy)]
pub struct LinePntr{
    pub line:usize
}
impl LinePntr{
    pub fn new(id:usize) -> LinePntr{
        LinePntr{line:id}
    }
}
#[derive(Clone, Debug, Copy)]
pub struct FilePntr{
    pub file:usize
}
impl FilePntr{
    pub fn new(id:usize) -> FilePntr{
        FilePntr{file:id}
    }
}
#[derive(Clone, Debug, Copy)]
pub struct LocationPntr{
    pub line:usize,
    pub block:usize,
}
impl LocationPntr{
    pub fn new(id:usize, id2:usize) -> LocationPntr{
        LocationPntr{line:id, block:id2}
    }
    pub fn from_line(orig:LinePntr, loc:usize) -> LocationPntr{
        LocationPntr{line:orig.line, block:loc}
    }
}
#[derive(Clone, Debug, Copy)]
pub struct StackPntr{
    pub block:usize,
}
impl StackPntr{
    #[allow(dead_code)]
    pub fn new(id:usize) -> StackPntr{
        StackPntr{block:id}
    }
}
#[derive(Clone, Debug)]
pub enum Data{
    Bool(bool),
    Int(i32),
    Unsigned(usize),
    Double(f64),
    Num(i128),
    Char(char),
    Func(FnPntr),
    Abstr(Vec<Vec<Code>>, bool),//Second bool = void. See "func". 
    List(LinePntr, Type),
    Tup(LinePntr, Vec<Type>, usize),//Tuple of size usize. 
    Ref(Pntr),
    Type(Type),
    Subdict(Subdict),
    Null,
    File(FilePntr),
}
#[derive(Clone, Debug, Copy)]
pub struct FnPntr{
    pub id:usize
}
impl FnPntr{
    pub fn new(id:usize) -> FnPntr{
        return FnPntr{id:id};
    }
}
#[derive(Clone, Debug)]
pub struct Func{
    pub exe:Vec<Vec<Code>>,
    pub var_names:Vec<String>,
    pub var_types:Vec<Type>,
    pub public:bool,
    pub void:bool,
    pub file:FilePntr,
}
impl Func{
    pub fn new(exe:Vec<Vec<Code>>, var_names:Vec<String>, var_types:Vec<Type>, public:bool, void:bool, file:FilePntr) -> Func{
        return Func{
            exe:exe,
            var_names:var_names,
            var_types:var_types,
            public:public,
            void:void,
            file:file,
        }
    }
}
impl Data{
    pub fn using(&self) -> Option<Pntr>{
        match self{
            Data::List(val, _) | Data::Tup(val, _,_)=>Some(Pntr::Line(*val)),
            Data::Ref(val)=>Some(*val),
            Data::Subdict(val)=>{Some(Pntr::Line(val.line))},
            Data::Func(val)=>{Some(Pntr::Func(*val))}
            _=>None,
        }
    }
    pub fn format(&self, dict:&mut Dict)->String{
        match self{
            Data::Bool(data)=>format!("{}", data),
            Data::Int(data)=>format!("{}", data),
            Data::Unsigned(data)=>format!("{}", data),
            Data::Double(data)=>format!("{}", data),
            Data::Num(data)=>format!("{}", data),
            Data::Char(data)=>format!("{}", data),
            Data::Func(x)=>format!("Function ({:?})", x),
            Data::Abstr(data, _)=>format!("{:?}", data),
            Data::List(data, _)=>{
                let is_string = self.is_string();
                let mut result:String = "".to_string();
                if is_string{
                    result.push('\"');
                    if dict.len(*data) == 0{
                        return "\"\"".to_string();
                    }
                }
                else{
                    result.push('[');
                    if dict.len(*data) == 0{
                        return "[]".to_string();
                    }
                }
                for line in dict.get_pntr_vec(*data){
                    result.push_str(&(line).format(dict));
                    if !is_string{
                        result.push(',');
                    }
                }
                if is_string{
                    result.push('\"');
                }
                else{
                    result.pop();
                    result.push(']');
                }
                return result;
            }
            _=>"null".to_string(),
        }
    }
    pub fn is_string(&self)-> bool{
        if let Data::List(_, typ) = self{
            if typ == (&Type::Char){
                return true;
            }
        }
        return false;
    }
    pub fn to_type(&self) -> Type{
        match self{
            Data::Bool(_)=>Type::Bool,
            Data::Int(_)=>Type::Int,
            Data::Unsigned(_)=>Type::Unsigned,
            Data::Double(_)=>Type::Double,
            Data::Num(_)=>Type::Num,
            Data::Char(_)=>Type::Char,
            Data::Func(_)=>Type::Func,
            Data::Abstr(_,_)=>Type::Abstr,
            Data::List(_,val2)=>Type::List(Box::new(val2.clone())),
            Data::Tup(_, types, size)=>Type::Tup(types.clone(), *size),
            Data::Null=>Type::Null,
            Data::Type(_)=>Type::Type,
            Data::Subdict(x)=>{Type::Subdict(x.id.clone())}
            Data::Ref(_)=>Type::Ref,
            Data::File(_)=>Type::File,
        }
    }
}
