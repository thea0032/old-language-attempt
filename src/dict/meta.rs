/*
use crate::dict::*;
use crate::dict::keyword::*;
use crate::operations;
#[derive(Clone, Debug)]
pub enum Instr{
    AddToWs(Addend),
    Exe(Vec<Code>),
    Ins(Vec<Code>),
}
impl Instr{
    pub fn exe(&mut self, input:&mut Vec<Code>, dict:&mut Dict, pos:usize, result:&mut Option<Data>, code_res:&mut Vec<Code>){
        match self{
            Instr::AddToWs(val)=>{operations::meta::add_to_ws(val.clone(), pos, input, dict)},
            Instr::Exe(val)=>{operations::meta::run(val.clone(), result, dict)},
            Instr::Ins(val)=>{operations::meta::ins(val.clone(), code_res, dict)},
        }
    }
}
#[derive(Clone, Debug)]
pub enum Addend{
    One(bool),
    All(bool),
    To(bool, KwID),
    This
}
#[derive(Clone, Debug)]
pub struct Behavior{
    out:Vec<Instr>,
}
impl Behavior{
    pub fn new() -> Behavior{
        return Behavior{out:vec![]};
    }
    pub fn get(&self) -> Vec<Instr>{
        return self.out.clone();
    }
    pub fn push(&mut self, adding:Instr){
        self.out.push(adding);
    }
}
pub struct BStrip{
    id:Vec<KwID>,
    behavior:Vec<Option<Behavior>>,
}
impl BStrip{
    pub fn new() -> BStrip{BStrip{id:vec![], behavior:vec![]}}
    pub fn get(&self, x:KwID) -> Behavior{
        for i in 0..self.id.len(){
            //println!("Comparing {:?} to {:?}", x, self.id[i].id);
            if self.id[i].id == x.id{
                return self.behavior[i].clone().unwrap_or_else(|| panic!("Behavior uninitialized!")); 
            }
        } panic!("Couldn't find a behavior for kwid {:?}", x)
    }
    pub fn pos(&self, x:KwID) -> usize{
        for i in 0..self.id.len(){
            //println!("Comparing {:?} to {:?}", x, self.id[i].id);
            if self.id[i].id == x.id{
                return i; 
            }
        } panic!("Couldn't find a behavior for kwid {:?}", x)
    }
    pub fn push(&mut self, id:KwID){
        self.id.push(id);
        self.behavior.push(None);
    }
    pub fn init(&mut self, id:KwID, behavior:Behavior){
        let temp = self.pos(id);
        self.behavior[temp] = Some(behavior);
    }
}
impl Dict{
    pub fn init_behavior(&mut self, behavior:Behavior, id:KwID){
        self.behaviors.init(id, behavior);
    }
    pub fn reserve_behavior(&mut self, phrase:String, priority:usize){
        let x = self.load_kw(crate::modules::generic(phrase, priority));
        self.behaviors.push(x);
    }
    pub fn load_kw(&mut self, keyword:Keyword) -> KwID{
        self.keywords.push(keyword);
        return KwID{id:self.keywords.len() - 1};
    }
    pub fn search_behavior(&self, id:KwID) -> Behavior{
        for i in 0..self.behaviors.id.len(){
            //println!("Comparing {:?} to {:?}", id, self.behaviors.id[i]);
            //println!("Comparing {:?} to {:?}", self.get_kw(id).phrase, self.get_kw(self.behaviors.id[i]).phrase);
            if self.behaviors.id[i].id == id.id{
                return self.behaviors.behavior[i].clone().unwrap_or_else(|| panic!("Behavior uninitialized!")).clone(); 
            }
        } panic!("Couldn't find a behavior for kwid {:?}", id)
    }
}
*/