use crate::operations::*;
use crate::new_ops;
pub fn equals(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    binary_basic(input, pos,dict, new_ops::equals);
}//If it equals something

pub fn neq(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    binary_basic(input, pos,dict, new_ops::neq);
}//If it isn't equal

pub fn leq(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    binary_basic(input, pos,dict, new_ops::leq);
}//If it's less than or equal

pub fn geq(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    binary_basic(input, pos,dict, new_ops::geq);
}//If it's greater than or equal

pub fn or(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    binary_basic(input, pos,dict, new_ops::or);
}//Or logic

pub fn and(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    binary_basic(input, pos,dict, new_ops::and);
}//and logic

pub fn not(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    unary_basic(input, pos,dict, new_ops::not);
}//not logic

pub fn greater(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    binary_basic(input, pos,dict, new_ops::greater);
}//Comparison

pub fn less(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    binary_basic(input, pos,dict, new_ops::less);
}//Comparison
