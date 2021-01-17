use crate::dict::*;
#[derive(Clone, Debug)]
pub struct Subdict{
    pub id:SubID,
    pub line:LinePntr,
}
pub struct SubStrip{
    pub names:Vec<String>,
    pub subs:Vec<SubTemplate>,
}
impl SubStrip{
    pub fn new() -> SubStrip{
        SubStrip{
            names:vec![],
            subs:vec![],
        }
    }
    pub fn get_names(&self) -> Vec<String>{
        return self.names.clone();
    }
}
impl Subdict{
    pub fn get(&mut self, superdict:&mut Dict, name:&str) -> Option<Pntr>{
        let mut template = superdict.subdicts.get_id(self.id.id);
        let res = template.search(name);
        if let Some(result) = res{
            return Some(Pntr::Location(LocationPntr::from_line(self.line, result)));
        } else {return None;}
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubID{
    id:usize,
}
impl SubID{
    pub fn new(id:usize) -> SubID{
        return SubID{id:id};
    }
    pub fn get(&self) -> usize{
        return self.id;
    }
}
impl SubStrip{
    pub fn search(&self, name:&str) -> Option<usize>{
        let mut i = 0;
        for line in &self.names{
            if name == line{
                return Some(i);
            }
            i += 1;
        }
        return None;
    }
    pub fn search_fields(&mut self, id:usize, name:&str) -> Option<usize>{
        let ref1 = &mut self.subs[id];
        return ref1.search(name);
    }
    pub fn get_type(&mut self, id1:usize, id2:usize) -> Type{
        let ref1 =  &mut self.subs[id1];
        return ref1.field_types[id2].clone();
    }
    pub fn get(&self, name:&str) -> SubTemplate{
        return self.subs[self.search(name).expect(&*format!("Couldn't find {}", name))].clone();
    }
    pub fn get_id(&self, id:usize) -> SubTemplate{
        return self.subs[id].clone();
    }
    pub fn add(&mut self, template:SubTemplate, name:String) -> SubID{
        let len = self.subs.len();
        self.subs.push(template);
        self.names.push(name);
        return SubID::new(len);
    }
    pub fn add_field_sub(&mut self, name:&str, names:Vec<String>, types:Vec<Type>){
        let searched = self.search(name).expect(&*format!("Couldn't find {}!", name));
        let refer = &mut self.subs[searched];
        refer.add_fields(names, types);
    }
}
#[derive(Clone, Debug)]
pub struct SubTemplate{
    pub field_names:Vec<String>,
    pub field_types:Vec<Type>,
    pub field_init:Vec<Data>,
}
impl SubTemplate{
    pub fn search(&mut self, name:&str) -> Option<usize>{
        let mut i = 0;
        for line in &self.field_names{
            if name == line{
                return Some(i);
            }
            i += 1;
        }
        return None;
    }
    pub fn add_fields(&mut self, mut names:Vec<String>,mut types:Vec<Type>){
        self.field_names.append(&mut names);
        self.field_types.append(&mut types);
    }
    pub fn import(names:Vec<String>, types:Vec<Type>, inits:Vec<Data>) -> SubTemplate{
        SubTemplate{
            field_names:names,
            field_types:types,
            field_init:inits,
        }
    }
}
impl Dict{
    pub fn add_template(&mut self, name:String, template:SubTemplate){
        self.subdicts.add(template, name);
    }
    pub fn get_template(&mut self, id:SubID) -> SubTemplate{
        return self.subdicts.subs[id.id].clone();
    }
    pub fn search_template(&mut self, name:&str) -> SubID{
        return SubID{
            id:self.subdicts.search(name).expect("Not found!")
        }
    }
    pub fn from_template(&mut self, id:SubID) -> Subdict{
        let data = self.subdicts.subs[id.id].field_init.clone();
        let pntr = self.bind_pntr_temp(data);
        return Subdict{
            id:id, 
            line:pntr
        }
    }
    pub fn get_field(&mut self, sub:&Subdict, name:&str) -> Data{
        let val =  self.subdicts.search_fields(sub.id.id, name).expect(&format!("Field {} not found!", name));
        let pntr = LocationPntr::from_line(sub.line, val);
        return self.get_pntr(Pntr::Location(pntr));
    }
    pub fn set_field(&mut self, sub:&Subdict, name:&str, data:Data){
        let val =  self.subdicts.search_fields(sub.id.id, name).expect(&format!("Field {} not found!", name));
        let pntr = LocationPntr::from_line(sub.line, val);
        self.set_pntr(Pntr::Location(pntr), data);
    }
}