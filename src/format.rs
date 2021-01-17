use crate::string;
pub fn handle(input:Vec<String>) -> Vec<Vec<String>>{
    clean(into_blocks(input))
}
pub fn into_blocks(mut input:Vec<String>) -> Vec<Vec<String>>{
    let mut result:Vec<String> = vec![];
    let mut buffer = "".to_string();
    let mut level = 0;
    let mut brace_level = 0;
    input.push(";".to_string());
    //debug(&format!("Input: {:?}", input));
    let pushing_buffer = input[input.len() - 1].clone().into_bytes();
    let adding_new = !(pushing_buffer[pushing_buffer.len() - 1] == ';' as u8);
    //debug(&format!("{}", adding_new));
    for mut line in input{
        line.insert(0, ' ');
        let mut start = 0;
        let x = comment_out_2(&line, level);//Comments things out. 
        level = x.1;
        line = x.0;
        line = comment_out(&line).to_string();//Comments things out.
        //debug(&format!("Your input: {}", line));
        while let Some(end) = detect_end(&line, start, &mut brace_level){
            //debug(&format!("Loop, end is {}", end));
            result.push(format!("{}{}", buffer, &line[start..end]));//Adds the buffer and the line to the result. 
            start = end + 1;//Updates the start for the if statement at the end. 
            buffer = " ".to_string();
            //debug(&format!("Start: {}", start));
        }
        if start < line.len(){
            buffer.push_str(&line[start..]);//Adds the rest of the line to the buffer. 
        }
    }
    if adding_new{
        result.push(buffer);
    }//The top stuff separates the lines out. The bottom stuff separates the commands out. 
    //debug(&format!("We have handled it, and the lines... {:?}", result));
    let mut grid:Vec<Vec<String>>= vec![];
    for mut line in result.clone(){
        if line == ""{
            continue;
        }
        let mut breaking = false;
        while &line[0..1] == " "{
            line = string::remove_first(line);
            if line == ""{
                breaking = true;
                break;
            }
        }
        if breaking{
            break;
        }//Basically, should get rid of all empty lines. 
        //debug(&format!("Separating {}", line));
        grid.push(separate(line));
    }
    grid
}
pub fn comment_out(input:&str) -> &str{
    if let Some(pos) = string::find_char(&input, "//"){
        return &input[..pos];
    }
    return input;
}
pub fn comment_out_2(input:&str, level:usize) -> (String, usize){
    let mut level_mut = level;
    let mut increment = 0;
    let mut result = "".to_string();
    let mut extra_chars = 0;
    while (increment as i32) < (input.len() as i32 - 1){
        let slice = &input[increment..increment+2];
        if slice == "/*"{
            level_mut += 1;
        }//This starts a block comment. 
        if slice == "*/" && level_mut > 0{
            level_mut -= 1;
            if level_mut == 0{
                extra_chars += 2;
            }
        }//This ends a block comment. You need to have a */ for every /*. 
        if level_mut == 0 {
            if extra_chars == 0{
            result.push_str(&input[increment..increment+1]);
            }
            else{
                extra_chars -= 1;
            }
        }
        increment += 1;
    }
    if level_mut == 0 && input.len() > increment{
        result.push_str(&input[increment..increment+1]);
    }
    result = string::remove_chars(result, extra_chars);
    return (result, level_mut);
}
pub fn detect_end(input:&str, start:usize, brace_level:&mut usize) -> Option<usize>{
    let mut i = start;
    let mut in_quotes = false;
    let mut slash_before = false;
    while i < input.len(){
        //debug(&format!("Loop, character is {}.", &input[i..i+1]));
        //debug(&format!("i = {}, in quotes: {}, slash before: {}", i, in_quotes, slash_before));
        match &input[i..i+1]{
            "\""=>{
                if !slash_before{
                    in_quotes = !in_quotes;
                }
            },
            "\\"=>{
                slash_before = !slash_before;
            },
            ";"=>{
                if !in_quotes && *brace_level == 0{
                    return Some(i);
                }
            },
            "{"=>{
                if !in_quotes{
                    *brace_level += 1;
                }
            },
            "}"=>{  
                if !in_quotes{
                    *brace_level -= 1;
                }
            }
            _=>{slash_before = false;}
        }
        i += 1;
    }
    return None;
}
pub fn separate(mut input:String) -> Vec<String>{
    //debug(&format!("Input: {}", input));
    while &input[0..1] == " "{
        input = string::remove_first(input);
    }//Removes all spaces from the beginning. 
    let mut instructions:Vec<String> = vec![];
    let mut buffer:String = "".to_string();
    let mut in_quotes = false;
    let mut bracket_level = 0;//Number of opening brackets that haven't been closed. 
    let mut par_level = 0;
    let mut preceding_slash = false;//Whether there's a slash that cancels out the quote. 
    let mut skip = 0;//Whether the next pattern will be skipped
    let mut brace_level = 0;
    for i in 0..input.len(){//For every character...
        if skip > 0{
            skip -= 1;
            //println!("WE SKIPPED SOMETHING!");
            continue;
        }
        let slice = &input[i..i+1];//Slice is the ith character. 
        let mut slice2 = "";
        if i != input.len() - 1{
            slice2 = &input[i..i+2];
        }//Slice2 is assigned here. This is used for things like ">=", "==", "||", etc. 
        if in_quotes{
            if quote_block(&slice, &mut in_quotes, &mut preceding_slash, &mut buffer, &mut brace_level, &mut bracket_level, &mut par_level){
                continue;
            }//See the function for more details. 
        }
        if bracket_block(slice, &mut bracket_level, &mut buffer, &mut instructions, &mut brace_level, &mut par_level, &mut in_quotes){
            continue;
        }//See the function for more details. 
        if brace_block(slice, &mut brace_level, &mut buffer, &mut instructions, &mut bracket_level, &mut par_level, &mut in_quotes){
            continue;
        }
        if paren_block(slice, &mut par_level, &mut buffer, &mut instructions, &mut brace_level, &mut in_quotes, &mut bracket_level){
            continue;
        }
        //debug(&format!("{}", bracket_level));
        if is_term2(slice2) && !in_quotes && bracket_level == 0 && brace_level == 0{
            if term_push_2(slice2, &mut buffer, &mut instructions){
                skip = 1;//Skip the next one. 
                continue;
            }
        }
        if is_terminator(slice) && !in_quotes && bracket_level == 0 && brace_level == 0{//If the character is a "terminator character".
            if term_push(&slice, &mut in_quotes, &mut buffer, &mut instructions){//Basically a block of code, intended to make the function a bit more manageable. 
                continue;
            }
        }
        buffer.push_str(slice);
    }
    if buffer != ""{
        instructions.push(buffer);//Puts the last bits (that might not have a terminator) into the buffer. 
    }
    instructions
}
pub fn bracket_block(slice:&str, bracket_level:&mut i32, buffer:&mut String, instructions:&mut Vec<String>, brace_level:&mut i32, par_level:&mut i32, in_quotes:&mut bool) -> bool{
    if slice == "]"{
        *bracket_level -= 1;
        if *par_level == 0 && *brace_level == 0 && *par_level == 0 && !*in_quotes && *bracket_level == 0{
            buffer.push_str(slice);
            instructions.push(buffer.clone());
            *buffer = "".to_string();
            return true;
        }
    }
    if slice == "["{
        if *par_level == 0 && *brace_level == 0 && *par_level == 0 && !*in_quotes && *bracket_level == 0{
            if buffer != ""{
                instructions.push(buffer.clone());//Adds the last bit of instructions before the first bit of braces. 
            }
            while buffer.len() > 0{
                buffer.pop();
            }//Clears buffer out. 
        }
        *bracket_level += 1;
        //debug(&format!("We went up a brace!"));
        buffer.push_str(slice);
        return true;
    }
    if *bracket_level != 0{
        buffer.push_str(slice);
        return true;
    }
    return false;
}//A block of the function, to make things easier. Handles brackets. "return true" is a substitute for "continue".
pub fn brace_block(slice:&str, brace_level:&mut i32, buffer:&mut String, instructions:&mut Vec<String>, bracket_level:&mut i32, par_level:&mut i32, in_quotes:&mut bool) -> bool{
    if slice == "}"{
        *brace_level -= 1;
    }
    if slice == "{"{
        if *par_level == 0 && *brace_level == 0 && *par_level == 0 && !*in_quotes && *bracket_level == 0{
            if buffer != ""{
                instructions.push(buffer.clone());//Adds the last bit of instructions before the first bit of braces. 
            }
            while buffer.len() > 0{
                buffer.pop();
            }//Clears buffer out. 
        }
        *brace_level += 1;
        //debug(&format!("We went up a brace!"));
        buffer.push_str(slice);
        return true;
    }
    if *brace_level != 0{
        buffer.push_str(slice);
        return true;
    }
    return false;
}//A block of the function, to make things easier. Handles brackets. "return true" is a substitute for "continue".
pub fn quote_block(slice:&str, in_quotes:&mut bool, preceding_slash:&mut bool, buffer:&mut String, brace_level:&mut i32, bracket_level:&mut i32, par_level:&mut i32) -> bool{
    if slice == "\"" && !*preceding_slash{
        *in_quotes = false;
        if *par_level == 0 && *brace_level == 0 && *par_level == 0 && !*in_quotes && *bracket_level == 0{
            buffer.push_str(slice);
            return true;
        }
    }//Handles quotes, so that terminators won't do anything bad. 
    if slice =="\\"{
        *preceding_slash = !*preceding_slash;
    }
    else{
        *preceding_slash = false;
    }//Handles slashes. 
    return false;
}//A block of "separate", built to make this easier to comprehend. Handles quotes. "return true" is a substitute for "continue". 
pub fn paren_block(slice:&str, par_level:&mut i32, buffer:&mut String, instructions:&mut Vec<String>, brace_level:&mut i32, in_quotes:&mut bool, bracket_level:&mut i32) -> bool{
    if slice == ")"{
        *par_level -= 1;
        if *par_level == 0 && *brace_level == 0 && *par_level == 0 && !*in_quotes && *bracket_level == 0{
            buffer.push_str(slice);
            instructions.push(buffer.clone());
            *buffer = "".to_string();
            return true;
        }
    }
    if slice == "("{
        if *par_level == 0 && *brace_level == 0 && *par_level == 0 && !*in_quotes && *bracket_level == 0{
            if buffer != ""{
                instructions.push(buffer.clone());//Adds the last bit of instructions before the first bit of braces. 
            }
            while buffer.len() > 0{
                buffer.pop();
            }//Clears buffer out. 
        }
        *par_level += 1;
        //debug(&format!("We went up a brace!"));
        buffer.push_str(slice);
        return true;
    }
    if *par_level != 0{
        buffer.push_str(slice);
        return true;
    }
    return false;
}//A block of the function, to make things easier. Handles brackets. "return true" is a substitute for "continue".
pub fn term_push(slice:&str, in_quotes:&mut bool, buffer:&mut String, instructions:&mut Vec<String>) -> bool{
    if slice == "\""{
        *in_quotes = true;//If there's a quotation mark, set in_quotes to true. 
        buffer.push_str(slice);//Adds the quotation mark on. 
        return true;
    }
    if buffer != ""{//If there's something in the buffer...
        //debug(&format!("Buffer: {}", buffer));
        instructions.push(buffer.clone());//Put buffer in instructions. 
        *buffer = "".to_string();//Reset buffer. 
    }
    if slice == " "{
        return true;
    }//So that we don't get places where there are only spaces. 
    instructions.push(slice.to_string());//Adds the terminator to the instructions. 
    return true;
}
pub fn term_push_2(slice2:&str, buffer:&mut String, instructions:&mut Vec<String>) -> bool{
    if buffer != ""{//If there's something in the buffer...
        //debug(&format!("Buffer: {}", buffer));
        if buffer != ""{
            instructions.push(buffer.clone());//Put buffer in instructions. 
        }
        *buffer = "".to_string();//Reset buffer. 
    }
    instructions.push(slice2.to_string());//Adds the terminator to the instructions. 
    return true;
}
const TERMS:[&str;23] = ["=", "(", ")", " ", "<", ">", "/", "\"", "\\", "+", "-", "*", "/", "&", "^", 
    "%", "$", "#", "@", "!", ":", "?", ","];
const TERMS2:[&str;16] = ["==", "!=", ">=", "<=", "&&", "||", "+=", "-=", "*=", "/=", "^=", "++", "--", "\\=", "\\+", "::"];
pub fn is_terminator(source:&str) -> bool{
    for line in TERMS.iter(){
        if &source == line{
            return true;
        }
    }
    return false;
}//Terminators. 
pub fn is_term2(source:&str) -> bool{
    for line in TERMS2.iter(){
        if &source == line{
            return true;
        }
    }
    return false;
}
fn clean(mut input:Vec<Vec<String>>) -> Vec<Vec<String>>{
    let mut i = 0;
    while i < input.len(){
        if input[i].len() == 0{
            input.remove(i);
        } else {
            i += 1;
        }
    }
    return input;
}