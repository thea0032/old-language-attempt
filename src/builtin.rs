/*
use crate::storage::*;
use crate::dict::*;
#[derive(Debug)]
pub enum BuiltIn{
    Print,
    Println,
    Input,
    Sto,
}
impl BuiltIn{
    pub fn phrase(&self)->String{
        match self{
            BuiltIn::Print=>"print".to_string(),
            BuiltIn::Println=>"println".to_string(),
            BuiltIn::Input=>"input".to_string(),
            BuiltIn::Sto=>"sto".to_string(),
        }
    }
    pub fn priority(&self)->usize{
        return 5;
    }
    pub fn get_variants(&self, buffer:&mut Vec<String>)->Vec<BuiltIn>{
        let result = vec![BuiltIn::Print, BuiltIn::Println, BuiltIn::Input, BuiltIn::Sto];
        for line in &result{
            buffer.push(line.phrase());
        }
        return result;
    }
    pub fn clone(&self) -> BuiltIn{
        match self{
            BuiltIn::Print=>BuiltIn::Print,
            BuiltIn::Println=>BuiltIn::Println,
            BuiltIn::Input=>BuiltIn::Input,
            BuiltIn::Sto=>BuiltIn::Sto,
        }
    }
}
pub fn convert_builtin(input:&str)->Option<Code>{
    let mut phrases:Vec<String> = vec![];
    let example = BuiltIn::Print;
    let result = example.get_variants(&mut phrases);
    for i in 0..result.len(){
        let line = &phrases[i];
        if input == line{
            return Some(Code::BuiltIn(result[i].clone()));
        }
    }
    return None;
}
impl BuiltIn{
    pub fn get_fn(&self)->fn(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, result:&mut Option<Data>){
        match self{
            BuiltIn::Input=>builtin_fns::input,
            BuiltIn::Print=>builtin_fns::print,
            BuiltIn::Println=>builtin_fns::println,
            BuiltIn::Sto=>builtin_fns::sto,
        }
    }
}
impl Code{
    pub fn op_builtin(&self, input:&mut Vec<Code>, dict:&mut Dict, pos:usize, result:&mut Option<Data>){
        if let Code::BuiltIn(val) = self{
            val.get_fn()(input, dict, pos, result);
        } else {panic!("Don't modify the code, you moron! Even I don't know how this works!");}
    }
}
mod builtin_fns{
    use crate::storage::*;
    use crate::dict::*;
    pub fn input(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _result:&mut Option<Data>){
        let inputted = crate::cmd_line::get_string(&crate::parse::evaluate(vec![input[pos+1].clone()], dict).to_str());
        let listchar:Vec<Data> = inputted.chars().map(|x| Data::Char(x)).collect();
        let result = Code::Object(Object::Literal(Data::List(listchar, Type::Char)));
        input.splice(pos..pos+2, vec![result]);
    }
    pub fn print(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _result:&mut Option<Data>){
        let out = crate::parse::evaluate(vec![input[pos+1].clone()], dict).to_str();
        print!("{}", out);
        input.splice(pos..pos+2, vec![]);
    }
    pub fn println(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _result:&mut Option<Data>){
        let out = crate::parse::evaluate(vec![input[pos+1].clone()], dict).to_str();
        println!("{}", out);
        input.splice(pos..pos+2, vec![]);
    }
    pub fn sto(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _result:&mut Option<Data>){
        if let Code::Object(Object::Parens(val)) = input[pos+1].clone(){
            let orig = crate::parse::evaluate(val[0].clone(), dict);
            if let Code::Type(val) = val[1][0].clone(){
                let result = orig.from_str(val);
                input.splice(pos..pos+2, vec![Code::Object(Object::Literal(result))]);
            } else {panic!("Expected type, found something else.");}
        } else {panic!("Expected parenthetical, found something else.");}
    }
}
*/