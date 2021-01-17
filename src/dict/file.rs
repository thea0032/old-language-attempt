use crate::dict::Pntr;
use crate::dict::*;
use crate::dict::storage::*;
pub struct Files{
    pub hist:Vec<usize>,
    pub names:Vec<String>, 
    pub files:Vec<File>,
    pub curr_file:usize,
}
impl Files{
    pub fn new(name:String) -> Files{
        Files{
            hist:vec![],
            names:vec![name],
            files:vec![File::new()],
            curr_file:0,
        }
    }
    pub fn new_file(&mut self, name:String){
        self.hist.push(self.curr_file);
        self.names.push(name);
        self.files.push(File::new());
        self.curr_file = self.files.len() - 1;
    }
    pub fn close_curr_file(&mut self){
        self.curr_file = self.hist.pop().unwrap();
    }
    pub fn name_to_id(&mut self, name:&str) -> Option<FilePntr>{
        for i in 0..self.names.len(){
            if name == &self.names[i]{
                return Some(FilePntr::new(i));
            }
        }
        return None;
    }
    pub fn curr_file(&self) -> FilePntr{
        return FilePntr::new(self.curr_file);
    }
    pub fn enter_file(&mut self, file:FilePntr){
        self.hist.push(self.curr_file);
        self.curr_file = file.file;
    }
}
pub struct File{
    pub names:Vec<String>,
    pub vals:Vec<Pntr>,
}
impl File{
    pub fn new() -> File{
        File{
            names:vec![],
            vals:vec![],
        }
    }
    //set, search, get, declare
    pub fn set(&mut self, val:Pntr, name:&str){
        if let Some(i) = self.search(name){
            self.vals[i] = val;
        } else {
            self.names.push(name.to_string());
            self.vals.push(val);
        }
    }
    pub fn search(&self, name:&str) -> Option<usize>{
        for i in 0..self.names.len(){
            if name == self.names[i]{
                return Some(i);
            }
        }
        return None;
    }
    pub fn get(&self, name:&str) -> Option<Pntr>{
        if let Some(val) = self.search(name){
            return Some(self.vals[val]);
        } else {
            return None;
        }
    }
}