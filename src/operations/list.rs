use crate::operations::*;
const AT:&str = "@";
pub fn at(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    if let Code::Object(Object::Literal(val)) = input[pos-1].clone(){
        match val{
            Data::List(val, _) | Data::Tup(val, _,_) => {
                let index = parse::evaluate(vec![input[pos+1].clone()], dict).to_unsigned(dict);
                let x = dict.get_pntr_vec(val)[index].clone();
                input.splice(pos-1..pos+2, vec![Code::Object(Object::Literal(x))]);
                return ();
            },
            _=>(),
        };
    }
    if let Code::Object(Object::Literal(Data::Subdict(val))) = &input[pos-1]{
        let field = if let Code::Object(Object::Variable(val)) = input[pos+1].clone(){
            val
        } else {panic!("Expected variable name, found {:?}!", input[pos+1])};
        let x =  dict.get_field(&val, &*field);
        input.splice(pos-1..pos+2, vec![Code::Object(Object::Literal(x))]);
    } else {
        panic!("Expected file or subdict, found {:?}", input[pos-1]);
    }
    panic!("Cannot get field of {:?}", input[pos-1]);
}//At. An operation that functions similarly to the [number] operation in most languages. [1, 2, 3, 4]@2 is 3, for example.  
pub fn push(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    let line = if let Code::Object(Object::Literal(Data::List(val, _))) = &input[pos - 1]{
        val.clone()
    } else {panic!("Expected variable, found {}!", crate::dict::display::display_code(&input[pos - 1], dict))};
    let data = parse::evaluate((&input[pos+1..]).to_vec(), dict);
    dict.push_element(Pntr::Line(line), data);
    *input = vec![];
}
pub fn add(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    let line = if let Code::Object(Object::Literal(Data::List(val, _))) = &input[pos - 1]{
        val.clone()
    } else {panic!("Expected variable, found {}!", crate::dict::display::display_code(&input[pos - 1], dict))};
    let at_pos = uth::find_end(input,pos, AT, dict);
    let data = parse::evaluate((&input[pos+1..at_pos]).to_vec(), dict);
    let pos = parse::evaluate((&input[at_pos+1..]).to_vec(), dict).to_unsigned(dict);
    let line = LocationPntr::from_line(line, pos);
    dict.push_element(Pntr::Location(line), data);
    *input = vec![];
}
pub fn pop(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    let line = if let Code::Object(Object::Literal(Data::List(val, _))) = &input[pos - 1]{
        val
    } else {panic!("Expected variable, found {}!", crate::dict::display::display_code(&input[pos - 1], dict))};
    dict.pop_element(Pntr::Line(*line));
    *input = vec![];
}
pub fn remove(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    let line = if let Code::Object(Object::Literal(Data::List(val, _))) = &input[pos - 1]{
        val
    } else {panic!("Expected variable, found {}!", crate::dict::display::display_code(&input[pos - 1], dict))};
    let pos = parse::evaluate((&input[pos+1..]).to_vec(), dict).to_unsigned(dict);
    let line = LocationPntr::from_line(*line, pos);
    dict.pop_element(Pntr::Location(line));
    *input = vec![];
}
pub fn clone(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    let line = if let Code::Object(Object::Literal(val)) = &input[pos+1]{val.clone()} else {panic!("Cannot clone {:?}", &input[pos+1])};
    let line = dict.clone_data_all(line);
    input.splice(pos..pos+2, vec![Code::Object(Object::Literal(line))]);
}
/*
pub fn copy(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    panic!("Not implemented yet!");
}
*/
pub fn len(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    let line = if let Code::Object(Object::Literal(Data::List(val, _))) = &input[pos - 1]{
        val
    } else {panic!("Expected variable, found {}!", crate::dict::display::display_code(&input[pos - 1], dict))};
    let len = dict.len(*line);
    input.splice(pos-1..pos+1, vec![Code::Object(Object::Literal(Data::Unsigned(len)))]);
}