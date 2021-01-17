use crate::dict::*;
use crate::dict::storage::*;
use crate::dict::code::*;
use crate::dict::typ::*;
pub fn make_template(input:&mut Vec<Code>, dict:&mut Dict, pos:usize,  _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    let name = if let Code::Object(Object::Variable(val)) = &input[pos+1]{
        val.to_string()
    } else {panic!("Expected variable name, found {}", crate::dict::display::display_code(&input[pos+1], dict))};
    let packet = if let Code::Object(Object::Literal(Data::Abstr(val, _))) = &mut input[pos+2]{
        val
    } else {panic!("Expected an abstraction literal, found {}", crate::dict::display::display_code(&input[pos+1], dict))};
    let mut field_names = Vec::<String>::new();
    let mut field_types = Vec::<Type>::new();
    let mut field_inits = Vec::<Data>::new();
    template_format(packet, &mut field_names, &mut field_types, &mut field_inits, dict);
    let mut new_field_inits = vec![];
    let mut i = 0;
    for typ in field_types.clone(){
        new_field_inits.push(field_inits[i].to(typ, dict));
        i += 1;
    }
    let new_template = crate::dict::subdict::SubTemplate::import(field_names, field_types, new_field_inits);
    dict.add_template(name, new_template);
    *input = vec![];
}
pub fn template_format(input:&mut Vec<Vec<Code>>, names:&mut Vec<String>, types:&mut Vec<Type>, init:&mut Vec<Data>, dict:&mut Dict){
    for line in input{
        let name = if let Code::Object(Object::Variable(val)) = &line[0]{
            val.clone()
        } else {panic!("Expected variable name, found {}", crate::dict::display::display_code(&line[0], dict))};
        names.push(name);
        let typ = if let Code::Object(Object::Literal(Data::Type(val))) = &line[2]{
            val.clone()
        } else {panic!("Expected type, found {}", crate::dict::display::display_code(&line[2], dict))};
        types.push(typ);
        let init_val = if let Code::Object(Object::Literal(val)) = &line[4]{
            dict.clone_data_all(val.clone())
        } else {Data::Null};
        init.push(init_val);
    }
}
pub fn make_sub(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    let template = if let Code::Object(Object::Literal(Data::Type(Type::Subdict(val)))) = &input[pos+1]{
        val.clone()
    } else {panic!("Expected subdictionary type, found {}", crate::dict::display::display_code(&input[pos+1], dict))};
    let sub = dict.from_template(template);
    input.splice(pos..pos+2, vec![Code::Object(Object::Literal(Data::Subdict(sub)))]);
}