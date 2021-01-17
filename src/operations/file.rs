use crate::operations::*;
use crate::dict::display::*;
const EXTENSION:&str = ".meta";
pub fn read(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, _:&mut Vec<Vec<Code>>, _:&mut i32){
    let mut file_id = if let Code::Object(Object::Variable(val)) = &input[pos + 1]{
        val.clone()
    } else {panic!("Expected variable, found {}", display_code(&input[pos + 1], dict))};
    file_id.push_str(EXTENSION);
    let output = crate::file::process_new_file(file_id.clone(), dict);
    let file_without_extension = (&file_id[0..file_id.len() - 5]).to_string();
    let temp = Data::File(dict.files.name_to_id(&file_id).unwrap());
    dict.set(&file_without_extension, temp);
    input.splice(pos..pos+2, vec![Code::Object(Object::Literal(output))]);
}
pub fn field(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, _:&mut Option<Data>, pvec:&mut Vec<Vec<Code>>, i:&mut i32){
    if let Code::Object(Object::Literal(Data::File(file))) = &input[pos - 1]{
        if let Code::Object(Object::Variable(val)) = &input[pos + 1]{
            let data= dict.get_file(val, *file);
            input.splice(pos - 1 .. pos + 2, vec![Code::Object(Object::Literal(data))]);
            reset_priorities(input, dict, pvec, i);
            return;
        }
        panic!("Expcted variable; found {}", display_code(&input[pos - 1], dict));
    }
    panic!("Cannot take field of {}", display_code(&input[pos - 1], dict));
}