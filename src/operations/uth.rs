use crate::operations::*;
use crate::dict::display::*;
use crate::debug::*;
fn build_vals(input:Vec<Vec<Code>>, dict:&mut Dict) -> Vec<Data>{
    let mut result:Vec<Data> = vec![];
    for line in input{
        debug(format!("INPUT: {:?}", line), 8);
        result.push(parse::evaluate(line, dict));
        debug(format!("PUSHING {:?}", result[result.len() - 1]), 8);
        //dict.display_heap();
    }
    return result;
}//Evalutates the lines of codes found in listerals and paren statements and converts them to regular literals. It's what converts [3+4, a] to Data::List([7, whatever a is]). 
pub fn listeral_conversion(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    let orig = input[pos].clone();
    if let Code::Object(Object::Listeral(val)) = orig{
        let result = build_vals(val, dict);
        let typ = assign_type(&result);
        input[pos] = Code::Object(Object::Literal(Data::List(dict.bind_pntr_temp(result), typ)));//TODO
    }
}//Turns a listeral into a literal. If it isn't a listeral, panics. That's because of the nature of how our code works. 
pub fn assign_type(input:&Vec<Data>) -> Type{
    let mut typ = Type::Null;
    for line in input{
        let temp = line.to_type();
        if typ != temp{
            if typ != Type::Null{
                return Type::Any;
            }
            typ = temp;
        }
    }
    return typ;
}
pub fn paren_conversion(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    let orig = input[pos].clone();
    if let Code::Object(Object::Parens(val)) = orig{
        let result = build_vals(val, dict);
        match result.len(){
            0=>{input.remove(pos);},
            1=>{input[pos] = Code::Object(Object::Literal(result[0].clone()))},
            _=>{input[pos] = Code::Object(Object::Literal(Data::List(dict.bind_pntr_temp(result), Type::Any)))},
        }
    }
}//Turns a paren into either a literal, a tuple, or nothing. 
const PUBLIC:&str = "pub";
const VOID:&str = "void";
const INLINE:&str = "inline";
pub fn define_function(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    let mut public = false;
    let mut void = false;
    let mut inline = false;
    let mut index = 0;
    if input[index] == (convert::convert(PUBLIC.to_string(), dict)){
        public = true;
        index += 1;
    }
    if input[index]== (convert::convert(VOID.to_string(), dict)){
        void = true;
        index += 1;
    }
    if input[index] == (convert::convert(INLINE.to_string(), dict)){
        inline = true;
    }
    let fn_name:String;
    if let Code::Object(Object::Variable(val)) = input[pos+1].clone(){
        fn_name = val;
    } else {panic!("Expected name, found {}", display_code(&input[pos+1], dict));}
    if input.len() > pos + 2{
        if let Code::Object(Object::Parens(val)) = input[pos+2].clone(){
            let mut names:Vec<String> = vec![];
            let mut types:Vec<Type> = vec![];
            separate(val, &mut names, &mut types);
            let exe:Vec<Vec<Code>>;
            if let Code::Object(Object::Literal(Data::Abstr(val2, _))) = input[pos+3].clone(){
                exe = val2;
            } else {panic!("Expected abstraction, didn't find it!")}
            let new_func = Func::new(exe, names, types, public, void, dict.files.curr_file());
            let new_id = dict.bind_pntr_func(new_func);
            dict.set(&fn_name, Data::Func(new_id));
        }
        if let Code::Object(Object::Literal(Data::Abstr(val, _))) = input[pos+2].clone(){
            dict.set(&fn_name, Data::Abstr(val, inline));
        }
    }
    *input = vec![];
}//This is how functions are defined. 
fn separate(input:Vec<Vec<Code>>, names:&mut Vec<String>, types:&mut Vec<Type>){
    for i in 0..input.len(){
        if let Code::Object(Object::Variable(val)) = input[i][0].clone(){
            names.push(val);
        }
        if input[i].len() >= 3{
            if let Code::Object(Object::Literal(Data::Type(val))) = input[i][2].clone(){
                types.push(val);
                continue;
            }
        }
        types.push(Type::Null);
    }
}//Parses out function input. Converts "["x", ":", "int", y, ":", "double"] to [x, y] and [int, double]. Stores them in the two vectors originally input. 
pub fn call_fn(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    if let Code::Object(Object::Literal(Data::Func(val))) = input[pos].clone(){
        let val = dict.get_pntr_func(val);
        let exe = val.exe;
        let names = val.var_names;
        let types = val.var_types;
        let public = val.public;
        let void = val.void;
        if input.len() > pos+1{
            if let Code::Object(Object::Parens(val2)) = input[pos+1].clone(){
                let vals = build_vals(val2, dict);
                dict.files.enter_file(val.file);
                if !public{
                    dict.new_function();
                } else {
                    dict.new_scope();
                } 
                let mut i = 0;
                while i < names.len() && i < vals.len(){
                    let temp = vals[i].to(types[i].clone(), dict);
                    dict.set(&names[i], temp);
                    i += 1;
                }
                let output:Option<Data> = 
                if void {
                    parse::parse_grid_void(exe, dict);
                    None
                } else {
                    parse::parse_grid_opt(exe, dict)//TODO This should be parse_grid
                };
                dict.drop_scope();
                dict.files.close_curr_file();
                let splicer = if let Some(val) = output{vec![Code::Object(Object::Literal(val))]}else{vec![]};
                input.splice(pos..pos+2, splicer);
            }
            else{
                panic!("Expected parenthesis, found something else!");
            }
        } else {panic!("Well, we found a pointer, but it didn't work out.")}
    } else {panic!("Well, we operated on an object, but it turned out that it isn't an object, so all of reality is wrong. ");}
}
pub fn call_abstr(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, pvec:&mut Vec<Vec<Code>>, i:&mut i32){
    if let Code::Object(Object::Literal(Data::Abstr(val, inline))) = &input[pos]{
        if *inline{
            let temp = val[0].clone();
            input.splice(pos..pos+1, temp.clone());
        } else {
            let temp = val.clone();
            dict.new_scope();
            let result = if let Some(x) = parse::parse_grid_opt(temp, dict){
                vec![Code::Object(Object::Literal(x))]
            } else {vec![]};
            dict.drop_scope();
            input.splice(pos..pos+1, result);
        }
        reset_priorities(input, dict, pvec, i);
    } else {panic!("An abstr was detected, but it turned out not to be an abstr!");}
}
pub fn var_unwrap(input:&mut Vec<Code>, pos:usize, dict:&mut Dict, _:&mut Option<Data>, pvec:&mut Vec<Vec<Code>>, i:&mut i32){
    if let Code::Object(Object::Variable(val)) = &input[pos]{
        input[pos] = Code::Object(Object::Literal(dict.get(val)));
        reset_priorities(input, dict, pvec, i);
    } else {panic!("A variable was found at pos {}, but, then, it turned out it wasn't a variable!", pos)}
}//The variable unwrap operator. Turns a variable into the literal it represents. 
pub fn find_end(input:&mut Vec<Code>, start:usize, end:&str, dict:&mut Dict) -> usize{
    for i in start..input.len(){
        if input[i] == crate::convert::convert(end.to_string(), dict){
            return i;
        }
    }
    panic!("No end for this statement found!");
}
