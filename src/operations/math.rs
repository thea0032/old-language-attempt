use crate::operations::*;
use crate::new_ops;
pub fn add(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    binary_basic(input, pos, dict, new_ops::add);
}//Adds stuff
pub fn sub(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    binary_basic(input, pos, dict, new_ops::subtract);
}//Subtracts stuff

pub fn times(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    binary_basic(input, pos, dict, new_ops::multiply);
}//Multiplies stuff

pub fn append(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    binary_basic(input, pos, dict, new_ops::append);
}//Appends
pub fn concat(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    binary_basic(input, pos, dict, new_ops::append_str);
}//Concatwhatevers

pub fn div(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    binary_basic(input, pos, dict, new_ops::divide);
}//divides

pub fn exp(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    binary_basic(input, pos, dict, new_ops::exp);
}//Exponents (not implemented yet)

pub fn modulus(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    binary_basic(input, pos, dict, new_ops::mod_);
}//Modular arithmetic (TBD)
pub fn cvt(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    //println!("Converting type of {:?} at {}", input, pos);
    let val2:Data;
    let val1 = if let Code::Object(Object::Literal(Data::Type(val))) = input[pos+1].clone(){
        if let Code::Object(Object::Literal(data)) = input[pos-1].clone(){
            val2 = data;
        } else { panic!("Expected data, found something else!")}
        val
    } else if let Code::Object(Object::Literal(Data::Type(val))) = input[pos-1].clone(){
        if let Code::Object(Object::Literal(data)) = input[pos+1].clone(){
            val2 = data;
        } else { panic!("Expected data, found something else!")}
        val
    } else {panic!("No type found!")};
    input.splice(pos-1..pos+2, vec![Code::Object(Object::Literal(val2.to(val1, dict)))]);
}//The conversion operator. Example: Converts true:int to 1. Converts 4:double to 4.0. 