pub fn find_char(source:&str, finding:&str) -> Option<usize>{
    let mut i = 0;
    while (i as i32) < ((source.len() + 1) as i32 ) - finding.len() as i32{
        if &source[i..i+finding.len()] == finding{
            return Some(i);
        }
        i += 1;
    }
    None
}
pub fn remove_first(s: String) -> String{
    match s.chars().next().map(|c| &s[c.len_utf8()..]){//This line copied!
        Some(result) => result.to_string(),
        None => "".to_string(),
    }
}
pub fn remove_chars(mut s:String, num:usize) -> String{
    for _ in 0..num{
        s = remove_first(s);
    }
    s
}
pub fn to_esc(source:char) -> char{
    match source{
        'n'=>'\n',
        'r'=>'\r',
        't'=>'\t',
        '\''=>'\\',
        '\"'=>'\"',
        '\\'=>'\'',
        '0'=>'\0',
        _=>panic!("Invalid escape char!"),
    }
}
pub fn remove_quotes(mut source:String)->String{
    let chars = source.clone().into_bytes();
    if chars.len() > 0 && chars[chars.len() - 1] == ('\"' as u8){
        source.pop();
    }
    if chars.len() > 0 && source.len() > 0 && chars[0] == ('\"' as u8){
        source.remove(0);
    }
    return source;
}
pub fn split_parse(chars:&mut Vec<u8>, input:&mut String)->Vec<String>{
    let mut result:Vec<String> = vec![];
        let mut buffer:String = "".to_string();
        let mut par_level = 0;
        let mut bracket_level = 0;
        let mut brace_level = 0;
        for i in 1..chars.len()-1{
            let slice = &input[i..i+1];
            update_levels(slice, &mut brace_level, &mut bracket_level, &mut par_level);
            if par_level == 0 && bracket_level == 0 && brace_level == 0 && slice == ","{
                result.push(buffer);
                buffer = "".to_string();
            }else{
                buffer.push_str(slice);
            }
        }
    if buffer != "" && buffer.clone().into_bytes()[0] != (')' as u8){
        result.push(buffer);
    }
    return result;
}//Separates things based on commas. 
pub fn update_levels(input:&str, brace_level:&mut usize, bracket_level:&mut usize, par_level:&mut usize){
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
}//Updates "levels". 