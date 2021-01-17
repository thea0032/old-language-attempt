use crate::dict::*;
#[derive(Clone)]
pub struct Keyword{
    pub phrase:String,
    pub priority:usize, 
    pub exe:fn(input:&mut Vec<Code>, dict:&mut Dict, pos:usize, result:&mut Option<Data>, pvec:&mut Vec<Vec<Code>>, pos:&mut i32)
}
impl std::fmt::Debug for Keyword{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Keyword")
         .field("phrase", &self.phrase)
         .field("priority", &self.priority)
         .finish()
    }
}