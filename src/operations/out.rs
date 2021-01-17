use crate::operations::*;
use crate::debug::debug;
pub fn print_this(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    if let Code::Object(Object::Literal(val)) = &input[pos+1]{
        debug(format!("Printing {:?}", input[pos+1]), 2);
        let mut formatted = val.format(dict);
        match formatted.len(){
            0|1=>{},
            val=>{
                if (&formatted[0..1] == "\"") && (&formatted[val-1..val] == "\""){
                    formatted.remove(0);
                    formatted.pop();
                }
            }
        }
        print!("{}", formatted)
    } else {panic!("Expected data, found something else!")}
    input.splice(pos..pos+2, vec![]);
}
pub fn println_this(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    print_this(input, dict, pos, &mut None, &mut vec![], &mut 0);
    println!("");
}