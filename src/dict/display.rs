use crate::dict::*;
pub fn display_code(code:&Code, dict:&Dict) -> String{
    match code{
        Code::Object(val)=>format!("{}", display_obj(val, dict)),
        Code::Keyword(val)=>format!("Keyword({:?})", dict.get_kw(*val).phrase),
    }
}
pub fn display_obj(obj:&Object, dict:&Dict) -> String{
    match obj{
        Object::Listeral(_)=>format!("Listeral"),
        Object::Literal(val)=>format!("Literal({:?})", val),
        Object::Parens(val)=>{
            match val.len(){
                0=>format!("Parens: empty"),
                _=>format!("Parens: 1({})", display_line(&val[0], dict))}},
        Object::Variable(val)=>format!("Var({:?})", val),
    }
}
pub fn display_line(code:&Vec<Code>, dict:&Dict) -> String{
    let mut functions:Vec<FnPntr> = vec![];
    let mut result:String = "[".to_string();
    for line in code{
        result.push_str(&display_code(line, dict));
        result.push(',');
        if let Code::Object(Object::Literal(Data::Func(val))) = line{
            functions.push(*val);
        }
    }
    result.push(']');
    for line in functions{
        result.push_str(&format!("\nFunction pointer detected: id {}", line.id));
        result.push_str(&format!("\nThis function is: {:?}", dict.fns[line.id]));
    }
    return result;
}
/*
pub fn display_vars(dict:&mut Dict){
    let x = &dict.vars;
    let y = &x.names;
    let z = &x.vals;
    for i in 0..y.len(){
        let temp = dict.get_pntr(z[i]);
        debug(format!("Var: {} Val: {:?}", y[i], display_obj(&Object::Literal(temp), dict)), 2);
    }
}
*/