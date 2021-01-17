use crate::dict::*;
use crate::parse;
use crate::dict::typ::*;
use crate::dict::code::*;
use crate::dict::storage::*;
use crate::convert::convert;
pub fn slash_assign(in1:&mut Vec<Code>, in2:&mut Vec<Code>, dict:&mut Dict) -> Vec<Code>{
    let in2_mod = vec![convert("\\".to_string(), dict), Code::Object(Object::Parens(vec![in2.clone()]))];
    *in2 = in2_mod;
    return assign(in1, in2, dict);
}
pub fn assign(in1:&mut Vec<Code>, in2:&mut Vec<Code>, dict:&mut Dict) -> Vec<Code>{
    //println!("Assigning {:?} to {:?}...", in1, in2);
    let mut state = SetState::None;
    let mut typ = Type::Any;
    let setting = left_assign(in1, dict, &mut state, &mut typ);
    let setter:Data;
    match state{
        SetState::Setting=>{
            setter = parse::evaluate(in2.clone(), dict).to(typ, dict);
        }, SetState::Letting=>{
            setter = parse::reduced_eval(in2.clone(), dict);
        }, SetState::Defing=>{
            let var = from_setter(setting.clone(), dict);
            if let Data::Null = var{
                setter = parse::evaluate(in2.clone(), dict).to(typ, dict);
            }
            else{
                return vec![];
            }//Default: 
        }, _=> {
            let typ = from_setter_type(setting.clone(), dict,  &mut typ);
            if typ == Type::Abstr{
                setter = parse::evaluate(in2.clone(), dict);
            } else {
                setter = parse::evaluate(in2.clone(), dict).to(typ, dict);
            }
        }   
    }
    //debug(&format!("Setting {:?} to {:?}", name, setter));
    to_setter(setting, dict, setter);
    return vec![];
}
fn from_setter(setter:Setting, dict:&mut Dict) -> Data{
    match setter{
        Setting::Name(val)=>{
            return dict.get(&val);
        }, Setting::Pntr(val)=>{
            return dict.get_pntr(Pntr::Location(val));
        }
    }
}
fn from_setter_type(setter:Setting, dict:&mut Dict, typ:&mut Type) -> Type{
    match setter{
        Setting::Name(val)=>{
            return dict.get(&val).to_type();
        }, Setting::Pntr(_)=>{
            typ.clone()
        }
    }
}
fn to_setter(setter:Setting, dict:&mut Dict, data:Data){
    match setter{
        Setting::Name(val)=>{
            dict.set(&val, data);
        }, Setting::Pntr(val)=>{
            dict.set_pntr(Pntr::Location(val), data);
        }
    }
}
#[derive(PartialEq)]
enum SetState{
    Setting,
    Letting,
    Defing,
    None
}
#[derive(Clone)]
enum Setting{
    Name(String),
    Pntr(LocationPntr),
}
fn left_assign(input:&mut Vec<Code>, dict:&mut Dict, state:&mut SetState, typ:&mut Type) -> Setting{
    let set_id = convert("set".to_string(), dict);
    let let_id = convert("let".to_string(), dict);
    let def_id = convert("def".to_string(), dict);
    if input[0] == set_id {*state = SetState::Setting}
    if input[0] == let_id {*state = SetState::Letting}
    if input[0] == def_id {*state = SetState::Defing}
    if *state != SetState::None{
        input.remove(0);
    }//Keywords have been parsed!
    let name;
    if let Code::Object(Object::Variable(val)) = input[0].clone(){
        name = val;
    }
    else{
        panic!("Expected variable name, found {:?}.", display::display_code(&input[0], dict));
    }
    if input.len() >= 3{
        let one_id = convert(":".to_string(), dict);
        let two_id = convert("::".to_string(), dict);
        if input[1] == one_id{
            *typ = parse::parse_type(&mut (&input[2..]).to_vec(), dict);
        }
        if input[1] == two_id{
            return Setting::Pntr(find_field_type(input, dict, typ));
        }
    }
    return Setting::Name(name);
}
fn find_field_type(input:&mut Vec<Code>, dict:&mut Dict, typ:&mut Type) -> LocationPntr{
    if let Code::Object(Object::Variable(sub)) = &input[0]{
        if let Code::Object(Object::Variable(field)) = &input[2]{
            let new = dict.get(sub);
            if let Data::Subdict(val) = new{
                let id = val.id;
                let line = val.line;
                let location = dict.subdicts.search_fields(id.get(), field).expect(&*format!("Invalid field: {}", field));
                *typ = dict.subdicts.get_type(id.get(), location);
                return LocationPntr::from_line(line, location);
            }
        }
    }
    panic!("Not implemented yet!");
}