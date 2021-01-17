use crate::dict::*;
use crate::dict::code::*;
use crate::dict::typ::*;
use crate::dict::storage::*;
use crate::convert::*;
use crate::dict::meta::*;
use crate::debug::debug;
pub fn parse_grid(input:Vec<Vec<String>>, dict:&mut Dict) -> Data{
    for line in input{
        let line = convert_line(line, dict);
        dict.debug_heap();
        dict.debug_stack();
        dict.debug_vars();
        dict.new_line();
        debug(format!("Parsing {:?} (grid)", display::display_line(&line, dict)), 9);
        if let Some(val) = parse(line, dict){
            return val;
        }
    }
    panic!("Reached end of code without output!");
}
pub fn parse_grid_opt(input:Vec<Vec<Code>>, dict:&mut Dict) -> Option<Data>{
    for line in input{
        dict.debug_heap();
        dict.debug_stack();
        dict.debug_vars();
        dict.new_line();
        debug(format!("Parsing {:?} (grid)", display::display_line(&line, dict)), 9);
        //println!("Parsing {:?} (grid)", display::display_line(&line, dict));
        if let Some(val) = parse(line, dict){
            return Some(val);
        }
    }
    return None;
}
pub fn parse_grid_void(input:Vec<Vec<Code>>, dict:&mut Dict){
    for line in input{
        dict.debug_heap();
        dict.debug_stack();
        dict.debug_vars();
        dict.new_line();
        debug(format!("Parsing {:?} (grid)", display::display_line(&line, dict)), 9);
        //println!("Parsing {:?} (grid void)", display::display_line(&line, dict));
        parse(line, dict);
    }
}
pub fn evaluate(mut input:Vec<Code>, dict:&mut Dict) -> Data{
    //println!("Evaluating {:?}", display::display_line(&input, dict));
    let mut priorities = make_priority_vec(input.clone(),vec![], dict, vec![Code::Object(Object::Literal(Data::Null))]);
    //println!("Priority vector (evaluation): {:?}", priorities);
    let mut result:Option<Data> = None;
    let mut i:i32 = 0;
    while (i as usize) < priorities.len(){
        let line = priorities[i as usize].clone();
        //println!("EVAL: Doing priority {}: {}", i, display::display_line(&line, dict));
        while skeleton(&mut input, dict, &line, &mut result, &mut priorities, &mut i){
            //println!("Line is {}", display::display_line(&input, dict));
            //println!("Changed input to {:?}", input);
            if result.is_some(){
                //println!("RETURNED");
                return result.unwrap();
            }
        }
        i += 1;
    }
    return Data::Null;
}
pub fn parse_type(input:&mut Vec<Code>, dict:&mut Dict) -> Type{
    //println!("Input: {:?}", input);
    if input.len() == 0{
        return Type::Null;
    }
    if let Code::Object(Object::Literal(Data::Type(mut begin_type))) = input[0].clone(){
        input.remove(0);
        match begin_type.get_next(){
            Next::Some(i)=>{
                for j in 0..i{
                    if begin_type.field_is_null(j){
                        begin_type.add_to_field(j, parse_type(input, dict));
                    }
                }
                //println!("Type is... {:?}", begin_type);
                return begin_type;
            },
            Next::None=>{
                //println!("Type is... {:?}", begin_type);
                return begin_type;
            }
        }
    }
    if let Code::Object(Object::Parens(mut val)) | Code::Object(Object::Listeral(mut val)) = input[0].clone() {
        let mut result:Vec<Code> = vec![];
        for i in 0..val.len(){
            result.push(Code::Object(Object::Literal(Data::Type(parse_type(&mut val[i], dict)))));
        }
        input.splice(0..1, result);
        return parse_type(input, dict);
    } else {panic!("Expected type, found {:?}!", input[0])}
}
pub fn reduced_eval(mut input:Vec<Code>, dict:&mut Dict) -> Data{
    //println!("Reduced eval {:?}", input);
    let mut priorities = vec![vec![convert("\\".to_string(), dict)]];
    //println!("Priority vector (reduced): {:?}", priorities);
    let mut result:Option<Data> = None;
    let mut i:i32 = 0;
    while (i as usize) < priorities.len(){
        let mut line = priorities[i as usize].clone();
        //println!("REDUCED: Doing priority {}: {}", i, display::display_line(&line, dict));
        //println!("Line is {}", display::display_line(&input, dict));
        while skeleton(&mut input, dict, &mut line, &mut result, &mut priorities, &mut i){}
        i += 1;
    }
    return Data::Abstr(vec![input], true);
}
pub fn parse(mut input:Vec<Code>, dict:&mut Dict) -> Option<Data>{
    //println!("Parsing {:?}", display::display_line(&input, dict));
    let mut priorities = make_priority_vec(input.clone(), vec![], dict, vec![]);
    //println!("Priority vector (parse): {:?}", priorities);
    let mut result:Option<Data> = None;
    let mut i:i32 = 0;
    while (i as usize) < priorities.len(){
        //println!("We should see this?");
        let line = &priorities[i as usize].clone();
        //println!("Line is {}", display::display_line(&input, dict));
        //println!("PARSE: Doing priority {}: {}", i, display::display_line(&line, dict));
        //let reference = i;
        //sets a reference, to see if i has changed. 
        while skeleton(&mut input, dict, line, &mut result, &mut priorities, &mut i){
            //println!("PARSE: Doing priority {}: {}", i, display::display_line(&line, dict));
            //println!("Line is {}", display::display_line(&input, dict));
            //println!("Changed input to {:?}", display::display_line(&input, dict));
            if result.is_some(){
                //println!("Returning {:?}", result);
                return result;
            }
            //println!("PARSE: We just finished priority {}: {}", i, display::display_line(&line, dict));
            //println!("Line is {}", display::display_line(&input, dict));
        }
        i += 1;
        //println!("i is {} and len is {}", i, priorities.len());
    }
    return None;
}
/*
    Takes an input and a vector of code, and runs through input. 
    If it finds something matching code's id (the == modifier), it'll use the code's exe operator and return true. 
    If it doesn't, it'll return false. False means "move on", and true means "do it again, just in case". 
*/
/*
pub fn parse_behavior(input:&mut Vec<Code>, dict:&mut Dict) -> Behavior{
    //println!("We're parsing a behavior!");
    let split = parse_behavior_split(input, dict);
    let mut result = Behavior::new();
    for line in split{
        result.push(to_instr(line, dict));
    }
    //println!("We've parsed a behavior!");
    //println!("Result is {:?}", result);
    return result;
}
pub fn to_instr(input:Vec<Code>, dict:&mut Dict) -> Instr{
    if let Some(x) = is_meta(&input[0], dict){
        if x == 0 {
            return Instr::AddToWs(to_addend(input[1..].to_vec(), dict));
        } else {
            if x == 1{
                return Instr::Ins(input[1..].to_vec());
            } else {
                return Instr::Exe(input[1..].to_vec());
            }
        }
    } else {panic!("Expected instruction, found {:?}", input[0])}
}
const RIGHT:&str = "right";
const TO:&str = "to";
const ONE:&str = "one";
const ALL:&str = "all";
pub fn to_addend(input:Vec<Code>, dict:&mut Dict)-> Addend{
    let boolean = match input[0]{
        Code::Keyword(id)=>{
            let phrase = dict.get_kw(id).phrase;
            phrase == RIGHT
        }
        _=>panic!("Expected keyword, found {:?}", input[0])
    };
    match input[1]{
        Code::Keyword(id)=>{
            match &*dict.get_kw(id).phrase{
                TO=>{let temp = if let Code::Keyword(x) = input[2]{x} else {panic!("Expected keyword, found {:?}", input[0])};
                    Addend::To(boolean, temp)},
                ONE =>Addend::One(boolean),
                ALL=>Addend::All(boolean),
                _=>panic!("Expected either \"to\", \"one\", or \"all\", found {}!", &*dict.get_kw(id).phrase)
            }
        }
        _=>{panic!("Expected keyword, found {:?}", input[1])}
    }
}

pub fn parse_behavior_split(input:&mut Vec<Code>, dict:&mut Dict) -> Vec<Vec<Code>>{
    let mut result:Vec<Vec<Code>> = vec![];
    let mut len = -1;
    for line in input{
        if is_meta(line, dict).is_some(){
            result.push(vec![]);
            len += 1;
        }
        if len == -1 {panic!("Expected meta, found {:?}", display::display_code(line, dict))}
        result[len as usize].push(line.clone());
    }
    return result;
}
const METAS:&[&str] = &["ws_add", "ins", "run"]; 
pub fn is_meta(input:&Code, dict:&mut Dict)->Option<usize>{
    if let Code::Keyword(val) = input{
        let compare = dict.get_kw(*val).phrase;
        let mut i = 0usize;
        for line in METAS{
            if &compare == line{return Some(i);}
            i += 1;
        } {return None;}
    } else {None}
}
pub fn parse_generic(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, data_result:&mut Option<Data>, behavior:&mut Vec<Instr>)-> Vec<Code>{
    let mut ins_result:Vec<Code> = vec![];
    dict.wspace_push(vec![]);
    for line in behavior{
        line.exe(input, dict, pos, data_result, &mut ins_result);
    }
    dict.wspace_pop();
    return ins_result;
}
const WS_PHRASE:&str = "ws";
use crate::convert;
pub fn parse_ws(input:&mut Vec<Code>, dict:&mut Dict){
    let p = &vec![convert::convert(WS_PHRASE.to_string(), dict)];
    while skeleton(input, dict, p, &mut None, &mut vec![], &mut 0){}
}
*/
fn skeleton(input:&mut Vec<Code>, dict:&mut Dict, code:&Vec<Code>, result:&mut Option<Data>, pvec:&mut Vec<Vec<Code>>, incr:&mut i32) -> bool{
    for i in 0..input.len(){
        if let Some(_) = input[i].search(code){
            //println!("Found {:?} in {:?}", input[i], input);
            let former_i = *incr;
            input[i].clone().operate(input, i, dict, result, pvec, incr);
            if former_i != *incr{
                return false;
            }
            return true;
        }
    }
    return false;
}
