pub fn handle(mut input:Vec<String>)-> Vec<Vec<String>>{
    //println!("Input: {:?}", input);
    input = comment_out(input);
    //println!("New input: {:?}", input);
    return sep_lines(input);
}
pub fn comment_out(input:Vec<String>) -> Vec<String>{
    let mut result:Vec<String> = vec![];
    let mut comment_level = 0;
    for line in input{
        //println!("Parsing line: {}", line);
        //println!("Comment level: {}", comment_level);
        if line.len() == 0{continue;}
        let mut builder = "".to_string();
        let mut skip = 0;
        let len = line.char_indices().count() - 1;
        for val in 0..len{
            //println!("      Bit {}", &line[val..val+2]);
            //println!("      Comment level: {}", comment_level);
            if skip > 0 {skip -= 1; continue;}
            match &line[val..val+2]{
                "//"=>{skip += 1;break;},
                "/*"=>{if comment_level == 0{result.push(builder.clone());};comment_level += 1;skip += 1; continue;},
                "*/"=>{if comment_level == 0{panic!("You're ending a block that didn't begin!")};
                    comment_level -= 1; 
                    if comment_level == 0{builder = "".to_string(); skip += 1; continue;}},
                _=>{},
            }
            builder.push_str(&line[val..val+1]);
        }
        if skip == 0{builder.push_str(&line[len..len+1]);}
        if comment_level == 0{result.push(builder);}
    }
    return result;
}
pub struct State{
    in_quotes:bool,
    slashed:bool,
    braces:usize,
    brackets:usize,
    parens:usize,
}
impl State{
    pub fn new() -> State{
        State{in_quotes:false,slashed:false,braces:0,brackets:0,parens:0}
    }
    pub fn update_state(&mut self, ch:&char){
        match ch{
            '\"'=>{if !self.slashed || !self.in_quotes{self.in_quotes = !self.in_quotes;} self.slashed = false;},
            '\\'=>{self.slashed = !self.slashed;},
            '{'=>{self.slashed = false;if !self.in_quotes{self.braces += 1;}},
            '}'=>{self.slashed = false;if !self.in_quotes{self.braces -= 1;}},
            '['=>{self.slashed = false;if !self.in_quotes{self.brackets += 1;}},
            ']'=>{self.slashed = false;if !self.in_quotes{self.brackets -= 1;}},
            '('=>{self.slashed = false;if !self.in_quotes{self.parens += 1;}},
            ')'=>{self.slashed = false;if !self.in_quotes{self.parens -= 1;}},
            _=>{self.slashed = false;},
        }
    }
    pub fn refresh_state(&mut self){
        self.in_quotes = false;
        self.slashed = false;
        self.braces = 0;
        self.brackets = 0;
        self.parens = 0;
    }
    pub fn can_terminate(&mut self) -> bool{
        return (!self.in_quotes) && (self.braces == 0) && (self.brackets == 0) && (self.parens == 0);
    }
    /*
    pub fn display(&self){
        println!("In quotes: {}", self.in_quotes);
        println!("Slashed: {}", self.slashed);
        println!("Brace level: {}", self.braces);
        println!("Bracket level: {}", self.brackets);
        println!("Par level: {}", self.parens)
    }
    pub fn debug(&self){
        debug(format!("In quotes: {}", self.in_quotes), 1);
        debug(format!("Slashed: {}", self.slashed), 1);
        debug(format!("Brace level: {}", self.braces), 1);
        debug(format!("Bracket level: {}", self.brackets), 1);
        debug(format!("Par level: {}", self.parens), 1);
    }
    */
}
fn sep_lines(input:Vec<String>) -> Vec<Vec<String>>{
    let mut result:Vec<Vec<String>> = vec![vec![]];
    let mut num = 0;
    let mut builder:String = "".to_string();
    let mut state = State::new();
    let mut skip = 0;
    for mut line in input{
        line.insert(0, ' ');
        //println!("Parsing line: {}", line);
        let ch_vec:Vec<char> = line.chars().collect();
        let temp = line.chars().count();
        for val in 0..temp{
            if skip > 0{
                skip -= 1; 
                continue;
            }
            if val < temp - 1{
                if is_term2(&line, val) && state.can_terminate(){
                    skip += 1;
                    if builder != ""{
                        result[num].push(builder);
                        builder = "".to_string();
                    }
                    result[num].push((&line[val..val+2]).to_string());
                    continue;
                }
            }
            let ch = ch_vec[val];
            let mut no_push = false;
            if is_terminator(ch) && state.can_terminate(){
                //println!("Is terminator: pos {} of {:?}", val, ch_vec);
                no_push = true;
                if builder != ""{
                    //println!("Pushing {:?}", builder);
                    result[num].push(builder);
                    builder = "".to_string();
                }
                match ch{
                    ' '=>{},
                    ';'=>{
                        num += 1;
                        result.push(vec![]);
                    },
                    '{' | '[' | '(' | '\"'=>{no_push = false},
                    _=>{
                        //println!("Pushing {}", ch);
                        result[num].push(ch.to_string());
                    },
                }
            }
            state.update_state(&ch);
            if !no_push{
                builder.push(ch);
            }
        }
    }
    if builder != "" {
        result[num].push(builder);
    }
    return clean(result);
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
fn is_terminator(input:char) -> bool{
    for line in TERMS{
        if &input == line{
            return true;
        }
    }
    return false;
}
fn is_term2(input:&String, pos:usize) -> bool{
    let vec = &input[pos..pos+2];
    for line in TERMS2{
        if line == &vec{return true;}
    }
    return false;
}
const TERMS:&'static[char] = &['=', '(', ')', ' ', '<', '>', '/', '\"', '\\', '+', '-', '*', '/', '&', '^', 
'%', '$', '#', '@', '!', ':', '?', ',', ';', '{', '}'];
const TERMS2:&'static[&str] = &["==", "!=", ">=", "<=", "&&", "||", "+=", "-=", "*=", "/=", "^=", "++", "--", "\\=", "\\+", "::"];