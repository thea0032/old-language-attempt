use crate::operations::*;
use crate::new_ops;
pub fn assign(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    raw_ops2::assign(&mut input[..pos].to_vec(), &mut input[pos+1..].to_vec(), dict);
    *input = vec![];
}//Assignment

pub fn passign(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    raw_ops2::slash_assign(&mut input[..pos].to_vec(), &mut input[pos+1..].to_vec(), dict);
    *input = vec![];
}//Parenthetical assignment

const ENDIF:&str = "?";
pub fn parse_if(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    //println!("Parsing if for {:?}", crate::dict::display::display_line(input, dict));
    let mut end = input.len();
    for i in pos..input.len(){
        if input[i] == crate::convert::convert(ENDIF.to_string(), dict){
            //println!("Found an end!");
            end = i;
            break;
        }
    }
    if end == input.len(){
        panic!("No end for the if statement found!");
    } 
    let temp = if let Some(x) = new_ops::if_parse(&mut input[pos+1..end].to_vec(), &mut input[end+1..end+2].to_vec(), dict){vec![Code::Object(Object::Literal(x))]} else {vec![]};
    input.splice(pos..end+2, temp);
}//If parsing
pub fn parse_else(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    //println!("Parsing {}", crate::dict::display::display_line(input, dict));
    let mut end = input.len();
    for i in pos..input.len(){
        if input[i] == crate::convert::convert(ENDIF.to_string(), dict){
            end = i;
            break;
        }
    }
    if end == input.len(){
        panic!("No end for the else statement found!");
    } 
    let temp = if let Some(x) = new_ops::else_parse(&mut input[pos+1..end].to_vec(), &mut input[end+1..end+2].to_vec(), dict){vec![Code::Object(Object::Literal(x))]} else {vec![]};
    //println!("Temp is {:?}", temp);
    input.splice(pos..end+2, temp);
    //println!("Input is {:?}", input);
}//else parsing

pub fn parse_while(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    let mut end = input.len();
    for i in pos..input.len(){
        if input[i] ==  crate::convert::convert(ENDIF.to_string(), dict){
            end = i;
        }
    }
    if end == input.len(){
        panic!("No end for the while statement found!");
    }
    let res = new_ops::while_parse(&mut input[pos+1..end].to_vec(), &mut input[end+1..end+2].to_vec(), dict);
    let temp = if let Some(x) = res{
        vec![Code::Object(Object::Literal(x))]
    } else { vec![]};
    input.splice(pos..end+2, temp);
}//While parsing

pub fn return_next(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, result:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    *result = Some(parse::evaluate((&input[pos+1..]).to_vec(), dict));
}//Returns everything after pos
pub fn return_this(input:&mut Vec<Code>, _:&mut Dict, pos:usize, result:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    if let Code::Object(Object::Literal(val)) = &input[pos]{
        *result = Some(val.clone());
    } else {panic!("You shouldn't see this message. This panic was called because the compiler is trying to return something that isn't data. This shouldn't happen.")}
}//returns pos
pub fn let_thing(_:&mut Vec<Code>, _:&mut Dict, _:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    panic!("Not implemented yet!");
}//Not implemented (should basically make something exist with value "null" or "type")

pub fn set_thing(_:&mut Vec<Code>, _:&mut Dict, _:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    panic!("You cannot set without assigning!");
}//Just has to be here for consistency's sake

pub fn def_thing(_:&mut Vec<Code>, _:&mut Dict, _:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    panic!("You cannot default without assigning!");
}//Just has to be here for consistency's sake
