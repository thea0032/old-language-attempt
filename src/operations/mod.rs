use crate::dict::*;
use crate::dict::typ::Type;
use crate::dict::code::*;
use crate::raw_ops2;
use crate::parse;
use crate::dict::storage::*;
use crate::convert;
use crate::dict::display::*;
pub mod math;
pub mod display;
pub mod logic;
pub mod control;
pub mod meta;
pub mod list;
pub mod uth;
pub mod out;
pub mod subdict;
pub mod file;
pub fn panic(_:&mut Vec<Code>, _:&mut Dict, _:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    panic!("Not implemented yet!");
}//Default panic; just has to be here for consistency's sake
pub fn binary_basic(input:&mut Vec<Code>, pos:usize, dict:&mut Dict, operation:fn(input:Vec<Data>, dict:&mut Dict)->Vec<Code>){
    if pos <= 0 || pos >= input.len(){
        panic!("Binary operators cannot be at the beginning or end of the line!");
    }
    //println!("Operation: {:?} \n Input: {:?} \n Pos: {}", operation, input, pos); TODO
    let op1;
    let op2;
    if let Code::Object(Object::Literal(val)) = input[pos - 1].clone(){
        op1 = val;
    } else if let Code::Object(Object::Literal(Data::Type(Type::Null))) = input[pos-1].clone(){
        op1 = Data::Null;
    } else {
        panic!("{:?} is not an object!", crate::dict::display::display_code(&input[pos - 1], dict));}
    if let Code::Object(Object::Literal(val)) = input[pos + 1].clone(){
        op2 = val;
    } else if let Code::Object(Object::Literal(Data::Type(Type::Null))) = input[pos+1].clone(){
        op2 = Data::Null;
    } else{panic!("{:?} is not a type!", crate::dict::display::display_code(&input[pos + 1], dict));}
    input.splice(pos-1..pos+2, operation(vec![op1, op2], dict));
}//A simple template for a lot of the functions. It's why most of them are a single line. 
pub fn unary_basic(input:&mut Vec<Code>, pos:usize, dict:&mut Dict, operation:fn(input:Vec<Data>, dict:&mut Dict)->Vec<Code>){
    if pos >= input.len(){
        panic!("Binary operators cannot be at the beginning or end of the line!");
    }
    let op;
    if let Code::Object(Object::Literal(val)) = input[pos + 1].clone(){
        op = val;
    } else {panic!("Cannot convert!");}
    input.splice(pos..pos+2, operation(vec![op], dict));
}//A simple template for what should end up to be a lot of the functions. It's why the "not" function is a single line. 
/*pub fn logic_splice(input:&mut Vec<Code>, pos:usize, dict:&mut dict, operation:fn(in1:&mut Vec<Code>, in2:&mut Vec<Code>, dict:&mut dict) -> Option<Data>) -> Option<Data>{
    for i in pos..input.len(){
        if input[i].is(&Code::Existing(bind_existing("?").unwrap())){
            if let Some(data) = operation(&mut input[pos+1..i].to_vec(), &mut input[i+1..i+2].to_vec(), dict){
                (*input).splice(pos..i+2, vec![Code::Object(Object::Literal(data))]);
            } else{(*input).splice(pos..i+2, vec![]);}
            break;
        }
    }
    //println!("We did it!");
    return None;
}*///Commented out because it didn't work out.
/* 
pub fn _call_function(input:&mut Vec<Code>, pos:usize, dict:&mut Dict) -> bool{
    if let Code::Object(Object::Literal(Data::Func(exe, names, types, public, void))) = input[pos].clone(){
        if input.len() > pos+1{
            if let Code::Object(Object::Parens(val2)) = input[pos+1].clone(){
                let vals = build_vals(val2, dict);
                if !public{
                    dict.new_function();
                } else {
                    dict.new_scope();
                } 
                let mut i = 0;
                while i < names.len() && i < vals.len(){
                    let temp = vals[i].to(types[i].clone(), dict);
                    dict.set(&names[i], temp);
                    i += 1;
                }
                let output:Option<Data> = 
                if void{
                    parse::parse_grid_void(exe, dict);
                    None
                } else {
                    parse::parse_grid_opt(exe, dict)
                };
                dict.drop_scope();
                let splicer = if let Some(val) = output{vec![Code::Object(Object::Literal(val))]}else{vec![]};
                input.splice(pos..pos+2, splicer);
                return true;
            }
            else{
                panic!("Expected parenthesis, found something else!");
            }
        }
    }
    return false;
}//Calls a function. It's complicated. Also deprecated. */
pub fn reset_priorities(input:&mut Vec<Code>, dict:&mut Dict, pvec:&mut Vec<Vec<Code>>, i:&mut i32){
    let last_line = pvec.pop().unwrap_or(vec![]);
    *pvec = add_to_pvec(pvec.clone(), input.clone(), last_line,  dict);
    *i = -1;//Sets i to -1. But that's fine, because it'll increase to 0 at the end of the loop. That's the only way to prevent it from skipping a priority. 
}
impl Code{
    pub fn operate(&self, input:&mut Vec<Code>, pos:usize, dict:&mut Dict, result:&mut Option<Data>, pvec:&mut Vec<Vec<Code>>, i:&mut i32){
        match self{
            Code::Keyword(val)=>(dict.get_kw(*val).exe)(input, dict, pos, result, pvec, i),
            Code::Object(val)=>val.operate(input, pos, dict, result, pvec, i),
        }
    }
}
impl Object{
    pub fn operate(&self, input:&mut Vec<Code>, pos:usize, dict:&mut Dict, result:&mut Option<Data>, pvec:&mut Vec<Vec<Code>>, i:&mut i32){
        match self{
            Object::Literal(val)=>val.operate(input, pos, dict, result, pvec, i),
            Object::Listeral(_)=>uth::listeral_conversion(input, dict, pos, result, pvec, i),
            Object::Parens(_)=>uth::paren_conversion(input, dict, pos, result, pvec, i),
            Object::Variable(_)=>uth::var_unwrap(input, pos, dict, result, pvec, i),
        }
    }
}
impl Data{
    pub fn operate(&self, input:&mut Vec<Code>, pos:usize, dict:&mut Dict, result:&mut Option<Data>, pvec:&mut Vec<Vec<Code>>, i:&mut i32){
        match self{
            Data::Abstr(_,_)=>uth::call_abstr(input, dict, pos, result, pvec, i),
            Data::Func(_)=>uth::call_fn(input, dict, pos, result, pvec, i),
            _=>control::return_this(input, dict, pos, result, pvec, i),
        }
    }
}