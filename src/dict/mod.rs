pub mod code;
pub mod typ;
pub mod vars;
pub mod storage;
pub mod keyword;
pub mod meta;
pub mod display;
pub mod subdict;
pub mod file;
use crate::dict::code::*;
use crate::dict::vars::*;
use crate::dict::storage::*;
use crate::dict::typ::Type;
use crate::dict::meta::*;
use crate::dict::subdict::*;
use crate::dict::file::*;
use crate::debug::debug;
pub struct Dict{
    pub keywords:Vec<keyword::Keyword>,
    heap:Vec<Line>,
    stack:Vec<Block>,
    fns:Vec<FnBlock>,
    vars:Vars,
    //pub behaviors:BStrip,
    pub env:Vec<WSpace>,
    pub line:usize,
    pub subdicts:SubStrip,
    pub files:Files,
}
pub struct WSpace{
    val:Vec<Vec<Code>>,
}
#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub struct KwID{
    pub id:usize,
}
impl Dict{
    pub fn new(filename:String) -> Dict{
        let mut blank_dict = Dict{
            heap:vec![], 
            keywords:vec![],
            stack:vec![],
            vars:Vars::new(),
            //behaviors:BStrip::new(),
            env:vec![],
            fns:vec![],
            subdicts:SubStrip::new(),
            line:0,
            files:Files::new(filename),
        };
        blank_dict.default_modules();
        return blank_dict;
    }
}
impl Dict{
    pub fn new_pntr_stack(&mut self) -> StackPntr{
        //safe
        let mut i = 0;
        while i < self.stack.len(){
            if self.stack[i].users == 0{
                if self.stack[i].line != self.line{
                    self.stack[i].line = self.line;
                    return StackPntr::new(i);
                }
            }
            i += 1;
        }
        let temp = self.line;
        self.stack.push(Block::new(temp));
        return StackPntr::new(i);
    }
    pub fn new_pntr_heap(&mut self) -> LinePntr{
        //safe
        let mut i = 0;
        while i < self.heap.len(){
            if self.heap[i].users == 0{
                if self.heap[i].line != self.line{
                    self.heap[i].line = self.line;
                    return LinePntr::new(i);
                }
            }
            i += 1;
        }
        let temp = self.line;
        self.heap.push(Line::new(temp));
        return LinePntr::new(i);
    }
    pub fn new_pntr_func(&mut self) -> FnPntr{
        //safe
        let mut i = 0;
        while i < self.fns.len(){
            if self.fns[i].users == 0{
                if self.fns[i].line != self.line{
                    self.fns[i].line = self.line;
                    return FnPntr::new(i);
                }
            }
            i += 1;
        }
        let temp = self.line;
        self.fns.push(FnBlock::new(temp, Func::new(vec![], vec![], vec![], false, false, FilePntr::new(0))));
        return FnPntr::new(i);
    }
    pub fn bind_pntr_func(&mut self, function:Func) -> FnPntr{
        let pntr = self.new_pntr_func();
        let block = &mut(self.fns[pntr.id]);
        block.alter(self.line, function);
        return pntr;
    }
    pub fn get_pntr_func(&mut self, pntr:FnPntr) -> Func{
        return self.fns[pntr.id].data.clone();
    }
    pub fn get_pntr(&self, pointer:Pntr) -> Data{
        //printed
        //println!("Begin get pointer");
        let x = match pointer{
            Pntr::Line(_) => panic!("To get a vector of data, use get_pntr_vec(pntr)!"),
            Pntr::Location(id) => self.heap[id.line].data[id.block].clone(),
            Pntr::Stack(id) => self.stack[id.block].data.clone(),
            Pntr::Func(_) => panic!("You shouldn't be using get_pntr on a function pointer.."),
        };
        //println!("End get pointer");
        return x;
    }
    pub fn get_pntr_vec(&self, pointer:LinePntr) -> Vec<Data>{
        //printed
        //println!("Begin get pointer");
        let x = self.heap[pointer.line].data.clone();
        //println!("End get pointer");
        return x;
    }
    pub fn set_pntr(&mut self, pointer:Pntr, data:Data){
        //printed
        //println!("Begin set pointer");
        match pointer{
            Pntr::Line(_) => panic!("Cannot set a vector to a singular bit of data!"),
            Pntr::Location(id) =>self.change_element(id, data),
            Pntr::Stack(id) => {self.stack[id.block].data = data},
            Pntr::Func(_) => panic!("You shouldn't be using set_pntr on a function pointer."),
        }
    }
    pub fn set_pntr_vec(&mut self, pointer:LinePntr, data:Vec<Data>){
        //printed
        //println!("Begin set pointer");
        self.remove_line(pointer.clone());
        self.add_line(pointer, data);
    }
    pub fn bind_pntr_temp(&mut self, data:Vec<Data>) -> LinePntr{
        let result = self.new_pntr_heap();
        self.set_pntr_vec(result.clone(), data);
        return result;
    }
    pub fn bind_pntr_stack(&mut self, data:Data) -> Pntr{
        let result = Pntr::Stack(self.new_pntr_stack());
        self.set_pntr(result.clone(), data);
        return result;
    }
}
const LINE_PLACEHOLDER:usize = 0;
impl Dict{
    fn add_line(&mut self, pointer:LinePntr, data:Vec<Data>){
        for line in &data{
            if let Data::List(val, _) = line{
                self.add_user(&Pntr::Line(val.clone()));
            }
        }
        self.heap[pointer.line].data = data;
    }
    fn remove_line(&mut self, pointer:LinePntr){
        let temp_line = self.heap[pointer.line].users;
        if temp_line != 0 {
            for line in &self.get_pntr_vec(pointer){
                if let Some(val) = line.using(){
                    self.drop_user_safe(&val);
                }
            }
        }
        self.heap[pointer.line] = Line::new(LINE_PLACEHOLDER);
    }
    pub fn pop_element(&mut self, pointer:Pntr){
        match pointer{
            Pntr::Line(val)=>{
                let len = self.len(val) - 1;
                self.drop_thing(Pntr::Location(LocationPntr::new(val.line, len)));
                self.heap[val.line].data.pop();
            },
            Pntr::Location(val)=>{
                self.drop_thing(pointer);
                self.heap[val.line].data.remove(val.block);
            },
            Pntr::Stack(_)=>{
                panic!("You shouldn't be using pop_element on a stack pointer!");
            },
            Pntr::Func(_) => panic!("You shouldn't be using pop_element on a function pointer!"),
        }
    }
    pub fn push_element(&mut self, pointer:Pntr, data:Data){
        match pointer{
            Pntr::Line(val)=>{
                let len = self.len(val);
                self.heap[val.line].data.push(data);
                self.add_thing(Pntr::Location(LocationPntr::new(val.line, len)));
            },
            Pntr::Location(val)=>{
                self.add_thing(pointer);
                self.heap[val.line].data.insert(val.block, data);
            },
            Pntr::Stack(_)=>{
                panic!("You shouldn't be using this for the stack!");
            },
            Pntr::Func(_)=>{
                panic!("You shouldn't be pushing an element to a function pointer!");
            }
        }

    }
    pub fn change_element(&mut self, pointer:LocationPntr, data:Data){
        self.drop_thing(Pntr::Location(pointer.clone()));
        self.heap[pointer.line].data[pointer.block] = data;
        self.add_thing(Pntr::Location(pointer));
    }
    fn drop_thing(&mut self, pointer:Pntr){
        if let Some(val) = self.get_pntr(pointer).using(){
                self.drop_user(&val);
            }
    }//Called after a pointer is deleted. Removes one user from the thing it points to. 
    fn add_thing(&mut self, pointer:Pntr){
        if let Some(val) = self.get_pntr(pointer).using(){
            self.add_user(&val);
        }
    }//Called after a pointer is created. Adds one user to the thing it points to. 
    pub fn len(&mut self, pointer:LinePntr) -> usize{
        return self.get_pntr_vec(pointer).len();
    }
}
impl Dict{
    pub fn set(&mut self, name:&str, data:Data){
        if name == "_"{
            return;//Tree 1 - an underscore means that we do nothing w/ data 
        } else if let Some(i) = self.search(&name){
            let temp = self.vars.vals[i];
            self.drop_user_safe(&temp);
            self.set_pntr(temp, data);
            self.add_user(&temp);//Tree 2 - we found something in the function scope
        } else if let Some(i) = self.files.files[self.files.curr_file].search(name){
            let temp = self.files.files[self.files.curr_file].vals[i];
            self.drop_user_safe(&temp);
            self.set_pntr(temp, data);
            self.add_user(&temp);//Tree 3 - we found something in the file
        } else if self.in_global(){
            let temp = self.bind_pntr_stack(data);
            self.files.files[self.files.curr_file].set(temp, name);
            self.add_user(&temp);
        } else {
            self.vars.names.push(name.to_string());
            let temp = self.bind_pntr_stack(data);
            self.vars.vals.push(temp);
            //self.add_thing(temp);//WARNING: This is a band-aid. It may or may not work.
            self.add_user(&temp);//Tree 5 - we didn't find anything, and we aren't inside the file's scope
        }
    }//Set for stuff on the stack - whenever there is no new data, or for small things. This includes references
    pub fn set_file(&mut self, name:&str, file:FilePntr, data:Data){
        if name == "_"{
            return;
        } else if let Some(i) = self.files.files[file.file].search(name){
            let temp = self.files.files[file.file].vals[i];
            self.drop_user_safe(&temp);
            self.set_pntr(temp, data);
            self.add_user(&temp);
        } else {
            let temp = self.bind_pntr_stack(data);
            self.files.files[file.file].set(temp, name);
            self.add_user(&temp);
        }
    }
    pub fn pntr_to_var(&mut self, name:&str, pntr:Pntr){
        if let Some(i) = self.search(&name){
            self.vars.names[i] = name.to_string();
            if let Some(val) = self.get_pntr(pntr).using(){
                self.add_user(&val);
            }
            self.add_user(&pntr);//WARNING: This is a band-aid. It may or may not work. 
            //self.add_user(&pntr);
            self.vars.vals[i] = pntr;
        } else {
            self.vars.names.push(name.to_string());
            self.add_user(&pntr);//WARNING: This is a band-aid. It may or may not work. 
            //self.add_user(&pntr);
            self.vars.vals.push(pntr);
        }
    }//Set for stuff on the heap
    pub fn get_min(&self) -> usize{
        if self.vars.min_level.len() == 0{
            return 0;
        }
        return self.vars.min_level[self.vars.min_level.len() - 1];
    }
    pub fn get_last_line(&self) -> usize{
        if self.vars.min_level.len() == 0{
            return 0;
        }
        return self.vars.scope[self.vars.scope.len() - 1];
    }
    pub fn get_global(&self) -> usize{
        if self.vars.min_level.len() == 0{
            return 0;
        }
        return self.vars.min_level[0];
    }
    pub fn search(&self, name:&str) -> Option<usize>{
        let mut i = self.vars.names.len();//self.min_level[self.min_level.len() - 1];
        while i > self.get_min(){
            i -= 1;
            //println!("Comparing {} to {}...", name, self.vars.names[i]);
            if name == self.vars.names[i]{
                return Some(i);
            }
        }
        return None;
    }
    pub fn get_stack(&self, name:&str) -> Pntr{
        //println!("Getting {:?}", name);
        if let Some(index) = self.search(name){
            //println!("Data: {:?}", self.vars.vals[index]);
            return self.vars.vals[index].clone();
        }
        panic!("{} was not found!", name);
    }
    pub fn get_heap(&self, name:&str) -> LinePntr{
        let x = self.get_stack(name);
        if let Some(val) = self.get_pntr(x).using(){
            if let Pntr::Line(val) = val{
                return val;
            } else {panic!("{:?} is not a line pointer!", val)}
        } else {panic!("This can only be used on lists! {:?} is not a list!", x)}
    }
    pub fn get(&self, name:&str) -> Data{
        //println!("Getting {:?}", name);
        if let Some(i) = self.search(name){
            //println!("Data: {:?}", self.vars.vals[index]);
            let temp = self.vars.vals[i].clone();
            return self.get_pntr(temp);
        }
        if let Some(val) = self.files.files[self.files.curr_file].get(name){
            return self.get_pntr(val);
        }
        return Data::Null;
    }
    pub fn get_file(&self, name:&str, file:FilePntr) -> Data{
        if let Some(i) = self.files.files[file.file].get(name){
            return self.get_pntr(i);
        }
        return Data::Null;
    }
    pub fn declare(&mut self, name:&str){
        self.set(name, Data::Null);
    }
    pub fn exists(&mut self, name:&str) -> bool{
        self.search(name).is_some()
    }
    pub fn new_scope(&mut self){
        self.vars.scope.push(self.vars.names.len());
    }
    pub fn drop_scope(&mut self){
        let mut i = self.vars.names.len();
        while i > self.vars.scope[self.vars.scope.len() - 1]{
            i -= 1;
            self.remove_last();
        }//Removes every variable created in this scope. 
        while self.vars.min_level.len() != 0 && self.vars.min_level[self.vars.min_level.len() - 1] >= self.vars.names.len(){
            self.vars.min_level.pop();
        }//Removes the minimum levels
        while self.vars.global_scope.len() != 0 && self.vars.global_scope[self.vars.global_scope.len() - 1] >= self.vars.scope.len(){
            self.vars.global_scope.pop();
        }//Removes the global scopes
        self.vars.scope.pop();
    }
    pub fn remove_last(&mut self){
        let temp = self.vars.vals[self.vars.vals.len() - 1].clone();
        let last = self.get_pntr(temp);
        if let Some(val) = last.using(){
            self.drop_user_safe(&val);
        }
        self.vars.vals.pop();
        self.vars.names.pop();
    }
    pub fn new_function(&mut self){
        self.new_scope();
        self.vars.min_level.push(self.vars.names.len());
    }
    pub fn new_file(&mut self){
        self.new_function();
        self.vars.global_scope.push(self.vars.scope.len());
    }
    pub fn get_file_scope(&mut self) -> usize{
        return self.vars.global_scope[self.vars.global_scope.len() - 1];
    }
    pub fn init(&mut self){
        self.new_file();
        self.default_modules();
    }
    pub fn in_global(&mut self) -> bool{
        return self.vars.global_scope[self.vars.global_scope.len() - 1] == self.vars.scope.len();
    }
    pub fn add_user(&mut self, pntr:&Pntr){
        //println!("Adding user of something...");
        match pntr{
            Pntr::Line(id)=>{
                self.heap[id.line].users += 1;
                if self.heap[id.line].users == 1{
                    for line in self.heap[id.line].data.clone(){
                        if let Some(val) = line.using(){
                            self.add_user(&val);
                        }
                    }
                }
            },
            Pntr::Location(id)=>{
                self.heap[id.line].users += 1;
                if self.heap[id.line].users == 1{
                    if let Some(val) = &self.heap[id.line].data[id.block].using(){
                        self.add_user(val);
                    }
                }
            },
            Pntr::Stack(id)=>{
                self.stack[id.block].users += 1;
                if self.stack[id.block].users == 1{
                    if let Some(val) = &self.stack[id.block].data.using(){
                        self.add_user(val);
                    }
                }
            },
            Pntr::Func(id)=>{
                self.fns[id.id].users += 1;
            },
        }
    }
    pub fn drop_user_safe(&mut self, pntr:&Pntr){
        match pntr{
            Pntr::Line(id)=>{
                if self.heap[id.line].users == 0{
                    return;
                }
                self.heap[id.line].users -= 1;
                if self.heap[id.line].users == 0{
                    for line in self.heap[id.line].data.clone(){
                        if let Some(val) = line.using(){
                            self.drop_user(&val);
                        }
                    }
                }
            },
            Pntr::Location(id)=>{
                if self.heap[id.line].users == 0{
                    return;
                }
                self.heap[id.line].users -= 1;
                if self.heap[id.line].users == 0{
                    for line in self.heap[id.line].data.clone(){
                        if let Some(val) = line.using(){
                            self.drop_user(&val);
                        }
                    }
                }},
            Pntr::Stack(id)=>{
                if self.stack[id.block].users == 0{
                    return;
                }
                self.stack[id.block].users -= 1;
                if self.stack[id.block].users == 0{
                    if let Some(val) = &self.stack[id.block].data.using(){
                        self.drop_user(val);
                    }
                }
            },
            Pntr::Func(id)=>{
                if self.fns[id.id].users == 0{
                    return;
                }
                self.fns[id.id].users -= 1;
            }
        }
    }
    pub fn drop_user(&mut self, pntr:&Pntr){
        match pntr{
            Pntr::Line(id)=>{
                if self.heap[id.line].users == 0{
                    panic!("Tried to drop line {:?}, which had no users!", id.line);
                }
                self.heap[id.line].users -= 1;
                if self.heap[id.line].users == 0{
                    for line in self.heap[id.line].data.clone(){
                        if let Some(val) = line.using(){
                            self.drop_user(&val);
                        }
                    }
                }
            },
            Pntr::Location(id)=>{
                self.heap[id.line].users -= 1;
                if self.heap[id.line].users == 0{
                    for line in self.heap[id.line].data.clone(){
                        if let Some(val) = line.using(){
                            self.drop_user(&val);
                        }
                    }
                }},
            Pntr::Stack(id)=>{
                self.stack[id.block].users -= 1;
                if self.stack[id.block].users == 0{
                    if let Some(val) = &self.stack[id.block].data.using(){
                        self.drop_user(val);
                    }
                }
            },
            Pntr::Func(id)=>{
                self.fns[id.id].users -= 1;
            }
        }
    }
    pub fn new_line(&mut self){
        self.line += 1;
    }//A solution to a potential memory leak problem. 
    pub fn get_type(&self, name:&str) -> Type{
        return self.get(name).to_type();
    }
}
pub struct Module{
    pub keywords:Vec<keyword::Keyword>,
}
use crate::modules;
impl Dict{
    pub fn load_module(&mut self, module:Module){
        for line in module.keywords{
            self.keywords.push(line);
        }
    }
    pub fn get_kw(&self, id:KwID) -> keyword::Keyword{
        return self.keywords[id.id].clone();
    }
    pub fn default_modules(&mut self){
        self.load_module(modules::actions());
        self.load_module(modules::math());
        self.load_module(modules::logic());
        self.load_module(modules::modifiers());
        self.load_module(modules::file());
        self.load_module(modules::dbg());
        self.load_module(modules::list());
        self.load_module(modules::out());
        self.load_module(modules::subdict());
        //println!("Done?");
    }
    pub fn debug_heap(&mut self){
        let mut i = 0;
        debug(format!("DISPLAYING HEAP"), 1);
        for line in &self.heap{
            debug(format!("  LINE: {}", i), 1);
            debug(format!("      DATA: {:?}", line.data), 1);
            debug(format!("      USERS: {}", line.users), 1);
            i += 1;
        }
        debug(format!("END OF HEAP DISPLAY"), 1);
    }
    pub fn debug_stack(&mut self){
        let mut i = 0;
        debug(format!("DISPLAYING STACK"), 1);
        for line in &self.stack{
            debug(format!("  LINE: {}", i), 1);
            debug(format!("      DATA: {:?}", line.data), 1);
            debug(format!("      USERS: {}", line.users), 1);
            i += 1;
        }
        debug(format!("END OF STACK DISPLAY"), 1);
    }
    pub fn debug_vars(&mut self){
        debug(format!("DISPLAYING VARS"), 1);
        for i in 0..self.vars.names.len(){
            debug(format!("  LINE: {}", i), 1);
            debug(format!("      NAME: {}", self.vars.names[i]), 1);
            debug(format!("      VALUE: {:?}", self.vars.vals[i]), 1);
        }
        debug(format!("END OF VARS DISPLAY"), 1);
    }
    pub fn display_heap(&mut self){
        let mut i = 0;
        println!("DISPLAYING HEAP");
        for line in &self.heap{
            println!("  LINE: {}", i);
            println!("      DATA: {:?}", line.data);
            println!("      USERS: {}", line.users);
            i += 1;
        }
        println!("END OF HEAP DISPLAY");
    }
    pub fn display_stack(&mut self){
        let mut i = 0;
        println!("DISPLAYING STACK");
        for line in &self.stack{
            println!("  LINE: {}", i);
            println!("      DATA: {:?}", line.data);
            println!("      USERS: {}", line.users);
            i += 1;
        }
        println!("END OF STACK DISPLAY");
    }
    pub fn display_vars(&mut self){
        println!("DISPLAYING VARS");
        for i in 0..self.vars.names.len(){
            println!("  LINE: {}", i);
            println!("      NAME: {}", self.vars.names[i]);
            println!("      VALUE: {:?}", self.vars.vals[i]);
        }
        println!("END OF VARS DISPLAY");
    }
    pub fn wspace_last(&mut self, i:usize) -> Vec<Code>{
        let mut x = self.env.len();
        if x == 0{panic!("Workspace not yet initialized!")};
        x -= 1;
        return self.env[x].val[i].clone();
    }
    pub fn wspace_push(&mut self, stuff:Vec<Vec<Code>>){
        self.env.push(WSpace{val:stuff});
    }
    pub fn wspace_pop(&mut self){
        self.env.pop();
    }
    pub fn wspace_add(&mut self, stuff:Vec<Code>){
        let mut x = self.env.len();
        if x == 0{panic!("Workspace not yet initialized!")};
        x -= 1;
        self.env[x].val.push(stuff);
    }
}
impl Dict{
    pub fn clone_pntr(&mut self, stuff:Pntr) -> Pntr{
        match stuff{
            Pntr::Line(val)=>{
                let temp = self.get_pntr_vec(val);
                let temp = self.bind_pntr_temp(temp);
                return Pntr::Line(temp);
            },
            _=>{
                let temp = self.get_pntr(stuff);
                let temp = self.bind_pntr_stack(temp);
                return temp;
            },
        }
    }
    pub fn clone_pntr_all(&mut self, stuff:Pntr) -> Pntr{
        match stuff{
            Pntr::Line(val)=>{
                let temp = self.get_pntr_vec(val);
                let temp = self.clone_vec_all(temp);
                let temp = self.bind_pntr_temp(temp);
                return Pntr::Line(temp);
            },
            _=>{
                let temp = self.get_pntr(stuff);
                let temp = self.clone_data_all(temp);
                let temp = self.bind_pntr_stack(temp);
                return temp;
            },
        }
    }
    pub fn clone_pntr_line(&mut self, stuff:LinePntr) -> LinePntr{
        let temp = self.get_pntr_vec(stuff);
        let temp = self.bind_pntr_temp(temp);
        return temp;
    }
    pub fn clone_data_all(&mut self, stuff:Data) -> Data{
        match stuff{
            Data::List(val,typ)=>{
                Data::List(self.clone_pntr_line(val), typ)
            },
            _=>stuff
        }
    }
    pub fn clone_vec_all(&mut self, stuff:Vec<Data>) -> Vec<Data>{
        let mut result = Vec::new();
        for stuff in stuff{
            result.push(self.clone_data_all(stuff));
        }
        return result;
    }
}
pub fn make_priority_vec(input:Vec<Code>, essentials:Vec<Code>, dict:&mut Dict, last_essentials:Vec<Code>) -> Vec<Vec<Code>>{
    //println!("Making priority vec!");
    let mut result:Vec<Vec<Code>> = vec![];
    let essentials = essentials.to_vec();
    let sum = vec![input, essentials].concat();
    for code in sum{
        let i = code.clone().priority(dict);
        if i == 100{
            continue;
        }
        while i >= result.len(){
            result.push(vec![]);
        }
        if search(code.clone(), result[i].clone()).is_none(){
            result[i].push(code.simplify());
        }
    }
    result.push(last_essentials);
    let mut i = 0;
    while i < result.len(){
        if result[i].len() == 0{
            result.remove(i);
            continue;
        }
        i += 1;
    }
    //println!("Finished priority vec!");
    return result;
}
pub fn add_to_pvec(existing:Vec<Vec<Code>>, new_additions:Vec<Code>, last_line:Vec<Code>, dict:&mut Dict) -> Vec<Vec<Code>>{
    make_priority_vec(new_additions, chain(existing), dict, last_line)
}
pub fn chain(orig:Vec<Vec<Code>>) -> Vec<Code>{
    let mut result:Vec<Code> = vec![];
    for v in &orig{
        for b in v{
            result.push(b.clone());
        }
    }
    return result;
}
fn search(input:Code, potential:Vec<Code>) -> Option<usize>{
    //println!("Searching...");
    for i in 0..potential.len(){
        if input.eq(&potential[i]){
            return Some(i);
        }
    }
    //println!("Done with search!");
    return None;
}