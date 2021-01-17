use crate::dict::*;
use crate::debug::debug;
use crate::string;
use crate::format2;
use crate::dict::typ::Type;
use crate::dict::code::*;
use crate::dict::storage::*;
use crate::dict::subdict::*;
use crate::dict::display::*;
pub fn convert(input:String, dict:&mut Dict) -> Code{
    //println!("Converting START{}END", input);
    if let Some(val) = convert_kw(&input, dict){
        //println!("{} is {:?}.", input, display_code(&val, dict));
        return val;
    }
    if let Some(val) = convert_literal(&input, dict){
        //println!("{} is {:?}.", input, display_code(&val, dict));
        return val;
    }
    if let Some(val) = convert_type(&input){
        //println!("{} is {:?}.", input, display_code(&val, dict));
        return val;
    }
    if let Some(val) = convert_sub(&input, dict){
        //println!("{} is {:?}.", input, display_code(&val, dict));
        return val;
    }
    //println!("{} is variable.", input);
    return convert_var(&input);
}

fn convert_kw(input:&str, dict:&Dict) -> Option<Code>{
    let mut counter = 0;
    for line in &dict.keywords{
        if input == line.phrase{
            return Some(Code::Keyword(KwID{id:counter}));
        }
        counter += 1;
    }
    return None;
}
fn convert_type(input:&str) -> Option<Code>{
    match input{
        "func"=>{return Some(Code::Object(Object::Literal(Data::Type(Type::Func))));},
        "int"=>{return Some(Code::Object(Object::Literal(Data::Type(Type::Int))));},
        "num"=>{return Some(Code::Object(Object::Literal(Data::Type(Type::Num))));},
        "unsigned"=>{return Some(Code::Object(Object::Literal(Data::Type(Type::Unsigned))));},
        "abstr"=>{return Some(Code::Object(Object::Literal(Data::Type(Type::Abstr))));},
        "bool"=>{return Some(Code::Object(Object::Literal(Data::Type(Type::Bool))));},
        "char"=>{return Some(Code::Object(Object::Literal(Data::Type(Type::Char))));},
        "double"=>{return Some(Code::Object(Object::Literal(Data::Type(Type::Double))));},
        "null"=>{return Some(Code::Object(Object::Literal(Data::Type(Type::Null))));},
        "list"=>{return Some(Code::Object(Object::Literal(Data::Type(Type::List(Box::new(Type::Null))))));}
        _=>(),
    }
    return None;
}
fn convert_var(input:&str) -> Code{
    return Code::Object(Object::Variable(input.to_string()));
}
fn convert_sub(input:&str, dict:&mut Dict) -> Option<Code>{
    let strs = &dict.subdicts.names;
    let mut i = 0;
    for line in strs{
        if line == input{
            return Some(Code::Object(Object::Literal(Data::Type(Type::Subdict(SubID::new(i))))));
        }
        i += 1;
    }
    return None;
}
/*
pub fn convert_literal_2(input:&str, _:&mut Dict) -> Option<Code>{
    let mut in_clone = input.to_string();
    while &in_clone[0..1] == " "{
        in_clone.remove(0);
    }
    while in_clone.len() != 0 && &in_clone[in_clone.len() - 1..] == " "{
        in_clone.pop();
    }//removes all spaces
    match &*in_clone{
        "true" => {return Some(Code::Object(Object::Literal(Data::Bool(true))));},
        "false" => {return Some(Code::Object(Object::Literal(Data::Bool(false))));},
        _=>(),
    }
    if let Ok(x) = in_clone.parse::<f64>(){
        return Some(Code::Object(Object::Literal(Data::Double(x))));
    }//double
    if let Ok(x) = in_clone.parse::<i128>(){
        //debug("Successfully converted!".to_string());
        return Some(Code::Object(Object::Literal(Data::Num(x))));
    }//int
    if &in_clone[0..1] == "\'"{
        if in_clone.len() == 3{
            return Some(Code::Object(Object::Literal(Data::Char(in_clone[1..2].chars().next().unwrap()))));
        }
        if in_clone.len() == 2{
            return Some(Code::Object(Object::Literal(Data::Char(' '))));
        }
        if in_clone.len() == 4{
            return Some(Code::Object(Object::Literal(Data::Char(string::to_esc(in_clone[2..3].chars().next().unwrap() as char)))));
        }
    }//char
    return None;
}*/
pub fn convert_literal(input:&str, dict:&mut Dict) -> Option<Code>{
    let mut in_clone = input.to_string();
    let mut chars = input.as_bytes().to_vec();
    while chars[0] == (' ' as u8){
        chars.remove(0);
        in_clone.remove(0);
    }//Removes all spaces
    while chars.len() != 0 && chars[chars.len() - 1] == (' ' as u8){
        chars.pop();
        in_clone.pop();
    }//Removes all spaces, part 2. 
    //debug(format!("Converting {}", in_clone));
    if in_clone == "true"{
        return Some(Code::Object(Object::Literal(Data::Bool(true))));
    }//bool, true
    if in_clone == "false"{
        return Some(Code::Object(Object::Literal(Data::Bool(false))));
    }//bool, false
    if let Ok(x) = in_clone.parse::<f64>(){
        return Some(Code::Object(Object::Literal(Data::Double(x))));
    }//double
    if let Ok(x) = in_clone.parse::<i128>(){
        //debug("Successfully converted!".to_string());
        return Some(Code::Object(Object::Literal(Data::Num(x))));
    }//int
    if chars[0] == ('\'' as u8){
        if chars.len() == 3{
            return Some(Code::Object(Object::Literal(Data::Char(chars[1] as char))));
        }
        if chars.len() == 2{
            return Some(Code::Object(Object::Literal(Data::Char(' '))));
        }
        if chars.len() == 4{
            return Some(Code::Object(Object::Literal(Data::Char(string::to_esc(chars[2] as char)))));
        }
    }//char
    if chars[0] == ('\"' as u8){
        debug(format!("String detected: {:?}", input), 2);
        let mut result:Vec<Vec<Code>> = vec![];
        for mut i in 1..chars.len() - 1{
            if chars[i] == ('\\' as u8){
                i += 1;
                result.push(vec![Code::Object(Object::Literal(Data::Char(string::to_esc(chars[i] as char))))]);
            }
            else{
                result.push(vec![Code::Object(Object::Literal(Data::Char(chars[i] as char)))]);
            }
        }
        return Some(Code::Object(Object::Listeral(result)));
    }//string, or ls(char)
    if chars[0] == ('{' as u8) && chars[chars.len()-1] == ('}' as u8){
        in_clone.remove(0);
        in_clone.pop();
        //println!("Abstraction literal detected!");
        let formatted = format2::handle(vec![in_clone]);
        let in_line = formatted.len() == 1;
        return Some(Code::Object(Object::Literal(Data::Abstr(convert_block(formatted, dict), in_line))));
    }
    return convert_list(&mut chars, &mut in_clone, dict);
}//Converts literals to values. 
pub fn convert_list(chars:&mut Vec<u8>, in_clone:&mut String, dict:&mut Dict) -> Option<Code>{
    if chars[0] == ('[' as u8){
        let res = split_parse(chars, in_clone);
        //debug(format!("Result from conversion of list: {:?}", res));
        if res.len() == 0{
            //debug("Empty list!".to_string());
            return Some(Code::Object(Object::Listeral(vec![])));
        }
        return Some(Code::Object(Object::Listeral(to_code(res, dict))));
    }
    if chars[0] == ('(' as u8){
        let to_str:String = (&chars[1..chars.len() - 1]).iter().map(|x| *x as char).collect();
        let handled = format2::handle(vec![to_str]);
        if handled.len() == 0 {
            return Some(Code::Object(Object::Parens(vec![])));
        }
        let firstline = handled[0].clone();//It's not going to have multiple lines. 
        let mut res:Vec<Vec<Code>> = vec![vec![]];
        let mut line_in_res = 0;
        for block in firstline{
            match &*block{
                ","=>{
                    res.push(vec![]);
                    line_in_res += 1;
                    continue;
                },
                _=>{
                },
            };
            let converted = convert(block, dict);
            res[line_in_res].push(converted);
        }
        return Some(Code::Object(Object::Parens(res)));
    }
    return None;
}
//An extension of "convert".  
pub fn update_levels(input:&str, brace_level:&mut usize, bracket_level:&mut usize, par_level:&mut usize, in_quotes:&mut bool, slashed:&mut bool){
    if input == "{"{
        *brace_level += 1;
    }
    if input == "}"{
        *brace_level -= 1;
    }
    if input == "["{
        *bracket_level += 1;
    }
    if input == "]"{
        *bracket_level -= 1;
    }
    if input == "("{
        *par_level += 1;
    }
    if input == ")"{
        *par_level -= 1;
    }
    if !*slashed && input == "\""{
        *in_quotes = !*in_quotes;
    }
    if input == "\\"{
        *slashed = !*slashed;
    } else {
        *slashed = false;
    }
}//Updates "levels". 
pub fn split_parse(chars:&mut Vec<u8>, input:&mut String)->Vec<String>{
    let mut result:Vec<String> = vec![];
        let mut buffer:String = "".to_string();
        let mut par_level = 0;
        let mut bracket_level = 0;
        let mut brace_level = 0;
        let mut in_quotes = false;
        let mut slashed = false;
        for i in 1..chars.len()-1{
            let slice = &input[i..i+1];
            update_levels(slice, &mut brace_level, &mut bracket_level, &mut par_level, &mut in_quotes, &mut slashed);
            if !in_quotes && par_level == 0 && bracket_level == 0 && brace_level == 0 && slice == ","{
                result.push(buffer.to_string());
                buffer = "".to_string();
            }else{
                buffer.push_str(slice);
            }
        }
    if buffer != "" && buffer.clone().into_bytes()[0] != (')' as u8){
        result.push(buffer.to_string());
    }
    return result;
}//Separates things based on commas. 
pub fn to_code(input:Vec<String>, dict:&mut Dict) -> Vec<Vec<Code>>{
    let mut result:Vec<Vec<Code>> = vec![];
        for line in input{
            let separated = crate::format::separate(line);
            let mut res_line:Vec<Code> = vec![];
            for bit in separated{
                res_line.push(convert(bit, dict));
            }
            result.push(res_line);
        }
    return result;
}
pub fn convert_block(input:Vec<Vec<String>>, dict:&mut Dict) -> Vec<Vec<Code>>{
    let mut result:Vec<Vec<Code>> = Vec::new();
    for line in input{
        result.push(convert_line(line, dict));
    }
    return result;
}
pub fn convert_line(input:Vec<String>, dict:&mut Dict) -> Vec<Code>{
    let mut result:Vec<Code> = Vec::new();
    for line in input{
        result.push(convert(line, dict));
    }
    return result;
}
//An extension of "convert".  
/*
Math(Math),
    Action(Action),
    Modifier(Modifier),
    BoolLogic(BoolLogic),
    Object(Object),
*/
