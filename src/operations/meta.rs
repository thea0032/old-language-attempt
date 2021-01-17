/*
use crate::operations::*;
use crate::dict::meta::*;
pub fn add_to_ws(adding:Addend, pos:usize, input:&mut Vec<Code>, dict:&mut Dict){
    let x:Vec<Code> = match adding{
        Addend::All(val)=>{
            if val {
                let x = (&input[pos+1..]).to_vec();
                input.splice(pos+1.., Vec::new());
                x
            } else {
                let x = (&input[..pos]).to_vec();
                input.splice(..pos, Vec::new());
                x}},
        Addend::One(val)=>{
            if val {
                let x = (&input[pos+1..pos+2]).to_vec();
                input.splice(pos+1..pos+2, Vec::new());
                x
            } else {
                let x = (&input[pos-1..pos]).to_vec();
                input.splice(pos-1..pos, Vec::new());
                x}},
        Addend::To(val, id)=>{
            let pos2 = if val{
                if let Some(val) = Code::Keyword(id).search(&input[pos+1..].to_vec()){val} else {panic!("Keyword {:?} not found in {:?}", id, &input[pos+1..])}
            } else {
                if let Some(val) = Code::Keyword(id).search(&input[..pos].to_vec()){val} else {panic!("Keyword {:?} not found in {:?}", id, &input[pos+1..])}
            };
            if pos > pos2{
                let x = (&input[pos+1..pos2]).to_vec();
                input.splice(pos+1..pos2+1, Vec::new());
                x
            } else {
                let x = (&input[pos2+1..pos]).to_vec();
                input.splice(pos2..pos, Vec::new());
                x
            }
        },
        _=>panic!("This shouldn't have happened! I only added this match arm for consistency!")
    };
    dict.wspace_add(x);
}
use crate::parse;
pub fn ins(mut code:Vec<Code>, out:&mut Vec<Code>, dict:&mut Dict){
    parse::parse_ws(&mut code, dict);
    for line in code{
        out.push(line);
    }
}
pub fn run(mut code:Vec<Code>, out:&mut Option<Data>, dict:&mut Dict){
    parse::parse_ws(&mut code, dict);
    *out = parse::parse(code, dict);
}
pub fn reserve_meta(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    let phrase:String = if let Code::Object(Object::Variable(val)) = &input[pos+1]{val.clone()} else {panic!("The keyword {:?} is already reserved!", input[pos+1])};
    let priority:usize = if let Code::Object(Object::Literal(val)) = &input[pos+2]{val.to_unsigned(dict)} else {panic!("Couldn't convert {:?} to a number!", input[pos+2])};
    dict.reserve_behavior(phrase, priority);
    *input = vec![];
}
pub fn make_meta(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    let id = if let Code::Keyword(id) = input[pos+1]{
        id
    } else {panic!("Expected keyword, found {:?}", input[pos+1])};
    let behavior = parse::parse_behavior(&mut input[pos+2..].to_vec(), dict);
    dict.init_behavior(behavior, id);
    *input = vec![];
}
pub fn generic(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, result:&mut Option<Data>, pvec:&mut Vec<Vec<Code>>, i:&mut i32){  
    if let Code::Keyword(val) = input[pos]{
        let mut behavior = dict.search_behavior(val).get();
        *input = parse::parse_generic(input, dict, pos, result, &mut behavior);
        reset_priorities(input, dict, pvec, i);
    }
}
pub fn ws(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    let index = parse::evaluate(vec![input[pos+1].clone()], dict).to_unsigned(dict);
    let x = dict.wspace_last(index);
    input.splice(pos..pos+2, x);
}
*/