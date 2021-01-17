use crate::operations::*;
pub fn display_heap(input:&mut Vec<Code>, dict:&mut Dict, _:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    *input = vec![];
    dict.display_heap();
}
pub fn display_stack(input:&mut Vec<Code>, dict:&mut Dict, _:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    *input = vec![];
    dict.display_stack();
}
pub fn display_vars(input:&mut Vec<Code>, dict:&mut Dict, _:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    *input = vec![];
    dict.display_vars();
}
pub fn display_all(input:&mut Vec<Code>, dict:&mut Dict, _:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    *input = vec![];
    dict.display_heap();
    dict.display_stack();
    dict.display_vars();
}
pub fn display_line(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    input.remove(pos);
    println!("{}", crate::dict::display::display_line(input, dict));
}