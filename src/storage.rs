/*
#[derive(Debug)]
pub enum Math{
    Add, 
    Append, 
    Conca,
    Subtract, 
    Multiply, 
    Divide,
    Exp,
    Mod,
}
#[derive(Debug)]
pub enum Action{
    Assign,
    PAssign,
    If,
    Else,
    While,
    Return,
    Run,
    Let,
    Set,
    Def,
    Fn,
}
#[derive(Debug)]
pub enum Modifier{
    Slash,
    Colon,
    Comma,
    At,
    Pub,
    Void,
    Inline,
    EndIf,
    Multi,
}
#[derive(Debug)]
pub enum BoolLogic{
    Equals,
    Neq,
    Leq,
    Geq,
    Less,
    Greater,
    And,
    Or,
    Not,
}
#[derive(Debug)]
pub enum Object{
    Variable(String),
    Literal(Data),
    Listeral(Vec<Vec<Code>>),
    Parens(Vec<Vec<Code>>),
}
impl Clone for Code{
    fn clone(&self) -> Code{
        match self{
            Code::Math(val)=> Code::Math(val.clone()),
            Code::Action(val)=> Code::Action(val.clone()),
            Code::Modifier(val)=> Code::Modifier(val.clone()),
            Code::BoolLogic(val)=> Code::BoolLogic(val.clone()),
            Code::Object(val)=> Code::Object(val.clone()),
            Code::Type(val) => Code::Type(val.clone()),
            Code::BuiltIn(val)=>Code::BuiltIn(val.clone()),
        }
    }
}
impl Clone for Math{
    fn clone(&self) -> Math{
        match self{
            Math::Add=>Math::Add,
            Math::Append=>Math::Append,
            Math::Conca=>Math::Conca,
            Math::Divide=>Math::Divide,
            Math::Exp=>Math::Exp,
            Math::Mod=>Math::Mod,
            Math::Multiply=>Math::Multiply,
            Math::Subtract=>Math::Subtract,
        }
    }
}
impl Clone for Action{
    fn clone(&self) -> Action{
        match self{
            Action::Assign=>Action::Assign,
            Action::Def=>Action::Def,
            Action::Else=>Action::Else,
            Action::Fn=>Action::Fn,
            Action::If=>Action::If,
            Action::Let=>Action::Let,
            Action::PAssign=>Action::PAssign,
            Action::Return=>Action::Return,
            Action::Run=>Action::Run,
            Action::Set=>Action::Set,
            Action::While=>Action::While,
        }
    }
}
impl Clone for Modifier{
    fn clone(&self) -> Modifier{
        match self{
            Modifier::Colon=>Modifier::Colon,
            Modifier::Comma=>Modifier::Comma,
            Modifier::Slash=>Modifier::Slash,
            Modifier::At=>Modifier::At,
            Modifier::Inline=>Modifier::Inline,
            Modifier::Pub=>Modifier::Pub,
            Modifier::Void=>Modifier::Void,
            Modifier::EndIf=>Modifier::EndIf,
            Modifier::Multi=>Modifier::Multi,
        }
    }
}
impl Clone for BoolLogic{
    fn clone(&self) -> BoolLogic{
        match self{
            BoolLogic::And=>BoolLogic::And,
            BoolLogic::Equals=>BoolLogic::Equals,
            BoolLogic::Geq=>BoolLogic::Geq,
            BoolLogic::Greater=>BoolLogic::Greater,
            BoolLogic::Leq=>BoolLogic::Leq,
            BoolLogic::Less=>BoolLogic::Less,
            BoolLogic::Neq=>BoolLogic::Neq,
            BoolLogic::Not=>BoolLogic::Not,
            BoolLogic::Or=>BoolLogic::Or,
        }
    }
}
impl Clone for Object{
    fn clone(&self) -> Object{
        match self{
            Object::Variable(val,) => Object::Variable(val.clone()),
            Object::Literal(val) => Object::Literal(val.clone()),
            Object::Listeral(val) => Object::Listeral(val.clone()),
            Object::Parens(val) => Object::Parens(val.clone()),
        }
    }
}
impl Clone for Data{
    fn clone(&self)-> Data{
        match self{
            Data::Bool(data)=>Data::Bool(data.clone()),
            Data::Int(data)=>Data::Int(data.clone()),
            Data::Unsigned(data)=>Data::Unsigned(data.clone()),
            Data::Double(data)=>Data::Double(data.clone()),
            Data::Num(data)=>Data::Num(data.clone()),
            Data::Char(data)=>Data::Char(data.clone()),
            Data::Func(data, inputs, types, public, void)=>Data::Func(data.clone(), inputs.clone(), types.clone(), *public, *void),
            Data::Abstr(data, void)=>{
                let cloned = data.to_vec();
            Data::Abstr(cloned, *void)},
            Data::List(data, data2)=>Data::List(data.clone(), data2.clone()),
            _=>Data::Null,
        }
    }
}
impl Clone for Type{
    fn clone(&self) -> Type{
        match self{
            Type::Bool=>Type::Bool,
            Type::Int=>Type::Int,
            Type::Unsigned=>Type::Unsigned,
            Type::Double=>Type::Double,
            Type::Num=>Type::Num,
            Type::Char=>Type::Char,
            Type::List(val)=>Type::List(val.clone()),
            Type::Func=>Type::Func,
            Type::Abstr=>Type::Abstr,
            Type::Null=>Type::Null,
            Type::Undef=>Type::Undef,
            Type::Tup(val)=>Type::Tup(*val),
        }
    }
}
impl Type{
    pub fn equals(&self, comparing:&Type) -> bool{
        match self{
            Type::Bool=>if let Type::Bool = comparing{true}else{false},
            Type::Int=>if let Type::Int = comparing{true}else{false},
            Type::Unsigned=>if let Type::Unsigned = comparing{true}else{false},
            Type::Double=>if let Type::Double = comparing{true}else{false},
            Type::Num=>if let Type::Num = comparing{true}else{false},
            Type::Char=>if let Type::Char = comparing{true}else{false},
            Type::List(val1)=>if let Type::List(val2) = comparing{val1.equals(val2)}else{false},
            Type::Func=>if let Type::Func = comparing{true}else{false},
            Type::Abstr=>if let Type::Abstr = comparing{true}else{false},
            Type::Null=>if let Type::Null = comparing{true}else{false},
            Type::Undef=>if let Type::Undef = comparing{true}else{false},
            Type::Tup(val1)=>if let Type::Tup(val2) = comparing{val1 == val2}else{false}
        }
    }
    pub fn phrase(&self) -> String{
        match self{
            Type::Func=>"func".to_string(),
            Type::Int=>"int".to_string(),
            Type::List(val)=>format!("list[{}]", val.phrase()),
            Type::Null=>"null".to_string(),
            Type::Num=>"num".to_string(),
            Type::Undef=>"undef".to_string(),
            Type::Unsigned=>"unsigned".to_string(),
            Type::Abstr=>"abstr".to_string(),
            Type::Bool=>"bool".to_string(),
            Type::Char=>"char".to_string(),
            Type::Double=>"double".to_string(),
            Type::Tup(val)=>format!("Tup({})", val),
        }
    }
}
impl Data{
    
}


impl Code{
    pub fn phrase(&self) -> String{
        match self{
            Code::Action(val)=>val.phrase(),
            Code::BoolLogic(val)=>val.phrase(),
            Code::Math(val)=>val.phrase(),
            Code::Modifier(val)=>val.phrase(),
            Code::Type(val) => val.phrase(),
            Code::Object(val)=>val.phrase(),
            Code::BuiltIn(val)=>val.phrase(),
        }
    }
}
impl Code{
    pub fn is(&self, comparing:&Code) -> bool{
        self.phrase() == comparing.phrase()
    }
    pub fn is_phrase(&self, comparing:&str) -> bool{
        self.phrase() == comparing
    }
    pub fn priority(&self) -> usize{
        match self{
            Code::Action(val)=>val.priority(),
            Code::BoolLogic(val) => val.priority(),
            Code::Math(val) => val.priority(),
            Code::Modifier(val) => val.priority(),
            Code::Object(val) => val.priority(),
            Code::BuiltIn(val) => val.priority(),
            Code::Type(_) => 100,
        }
    }
    pub fn simplify(&self) -> Code{
        if let Code::Object(val) = self{
            match val{
                Object::Listeral(_) =>{return Code::Object(Object::Listeral(vec![]));},
                Object::Literal(_) =>{return Code::Object(Object::Literal(Data::Null));},
                Object::Parens(_) =>{return Code::Object(Object::Parens(vec![]));},
                Object::Variable(_) =>{return Code::Object(Object::Variable("".to_string()));},
            }
        }
        return self.clone();
    }
}
/*
    Priorities... 
    1. Backslash!
    2. Assignment
    3. fn, let (shouldn't coexist)
    4. if, else, while
    5. functions and variables
    6. Parenthesis expressions
    7. Listerals
    8. keywords, such as "run". 
    9. exponents
    10. Multiplication/division/moculus
    11. Plus/minus
    12. Comparison (==, >=, <=, >, <, !=)
    13. not
    14. And, or
    15. Appending (++)
    16. concatenation (\+)
    17. Literals (just returns them though)
*/
*/