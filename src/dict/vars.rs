use crate::dict::storage::*;
#[derive(Clone, Debug)]
pub struct Vars{
    pub names:Vec<String>,
    pub vals:Vec<Pntr>,
    //pub types:Vec<Type>,
    pub scope:Vec<usize>,
    pub min_level:Vec<usize>,
    pub fns:Vec<Func>,
    pub global_scope:Vec<usize>,
}
impl Vars{
    pub fn new() -> Vars{
        Vars{
            min_level:vec![],
            names:vec![],
            scope:vec![],
            vals:vec![],
            fns:vec![],
            global_scope:vec![],
        }
    }
}