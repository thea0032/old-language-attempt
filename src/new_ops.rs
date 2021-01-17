use crate::debug::debug;
use crate::parse;
use crate::string;
use crate::format;
use crate::dict::*;
use crate::dict::typ::*;
use crate::dict::code::*;
use crate::dict::storage::*;
use crate::convert;
use std::convert::TryInto;
impl Data{
    pub fn not(&self, dict:&mut Dict) -> bool{
        !self.to_bool(dict)
    }
    pub fn and(&self, op2:&Data, dict:&mut Dict) -> bool{
        self.to_bool(dict) && op2.to_bool(dict)
    }
    pub fn or(&self, op2:&Data, dict:&mut Dict) -> bool{
        self.to_bool(dict) || op2.to_bool(dict)
    }/*
    pub fn xor(&self, op2:&Data) -> bool{
        self.to_bool() ^ op2.to_bool()
    }*/
}//Bool logic
impl Data{
    pub fn to(&self, var_type:Type, dict:&mut Dict) -> Data{
        match var_type{
            Type::Bool=>Data::Bool(self.to_bool(dict)),
            Type::Num=>Data::Num(self.to_num(dict)),
            Type::Char=>Data::Char(self.to_char(dict)),
            Type::Double=>Data::Double(self.to_float(dict)),
            Type::Int=>Data::Int(self.to_int(dict)),
            Type::Null|Type::Any=>self.clone(),
            Type::Unsigned=>Data::Unsigned(self.to_unsigned(dict)),
            Type::List(val)=>{
                let mut x = self.to_ls(dict).clone();
                for i in 0..x.len(){
                    x[i] = x[i].to(*(val.clone()), dict);
                }
                let pntr = dict.bind_pntr_temp(x);
                return Data::List(pntr, *val);
            }
            Type::Abstr=>{
                if let Data::Abstr(_,_) = self{
                    return self.clone();
                }
            Data::Abstr(vec![vec![Code::Object(Object::Literal(self.clone()))]], true)},
            Type::Tup(val,size)=>{
                Data::Tup(self.to_tuple(size, dict), val, size)},
            Type::Subdict(val)=>{
                if let Data::Subdict(x) = self{
                    if x.id == val{
                        return self.clone();
                    }
                }
                panic!("Cannot convert {:?} to {:?}", self, val);
            }
            _=>panic!("Conversion to type {:?} not implemented yet!", var_type),
        }
    }
    pub fn to_tuple(&self, size:usize, dict:&mut Dict)->LinePntr{
        let mut result:Vec<Data> = vec![];
        for _ in 0..size{
            result.push(self.clone());
        }
        
        return dict.bind_pntr_temp(result);
    }
    pub fn to_bool(&self, dict:&mut Dict) -> bool{
        match self{
            Data::Bool(val)=>*val,
            Data::Char(val) => {
                if *val == ' '{ 
                    return false;
                }
                return true;
                },
            Data::List(val, _) =>{
                if dict.len(*val) == 0{
                    return false;
                }
                return true;
            }
            Data::Num(val)=>val != &0,
            Data::Int(val)=>val != &0,
            Data::Unsigned(val)=>val != &0,
            _=>false,
        }
    }
    pub fn to_num(&self, dict:&mut Dict) -> i128{
        match self{
            Data::Bool(val)=>{
                if *val{1}
                else{0}
            },
            Data::Char(val) => *val as i128,
            Data::List(val, _) =>
                dict.len(*val) as i128,
            Data::Num(val)=>*val,
            Data::Int(val)=>*val as i128,
            Data::Unsigned(val)=>*val as i128,
            Data::Double(val) => val.floor() as i128,
            _=>0,
        }
    }
    pub fn to_int(&self, dict:&mut Dict) -> i32{
        match self{
            Data::Bool(val)=>{
                if *val{1}
                else{0}
            },
            Data::Char(val) => *val as i32,
            Data::List(val, _) =>{dict.len(*val) as i32},
            Data::Num(val)=>(*val % (4294967295/2)+1) as i32,
            Data::Int(val)=>*val,
            Data::Unsigned(val)=>(*val - 2147483648) as i32,
            Data::Double(val) => *val as i32,
            _=>0,
        }
    }
    pub fn to_float(&self, dict:&mut Dict) -> f64{
        match self{
            Data::Bool(val)=>{
                if *val{1.0}
                else{0.0}
            },
            Data::Char(val) => *val as u8 as f64,
            Data::List(val, _) =>dict.len(*val) as f64,
            Data::Num(val)=>*val as f64,
            Data::Int(val)=>*val as f64,
            Data::Unsigned(val)=>*val as f64,
            Data::Double(val) => *val,
            _=>0.0,
        }
    }
    pub fn to_unsigned(&self, dict:&mut Dict) -> usize{
        match self{
            Data::Bool(val)=>{
                if *val{1}
                else{0}
            },
            Data::Char(val) => *val as usize,
            Data::List(val, _) =>dict.len(*val) as usize,
            Data::Num(val)=>(*val % 4294967295) as usize,
            Data::Int(val)=>val.abs() as usize,
            Data::Unsigned(val)=>*val as usize,
            Data::Double(val) => val.abs() as usize,
            _=>0,
        }
    }
    pub fn to_char(&self, dict:&mut Dict) -> char{
        match self{
            Data::Bool(val)=>{if *val{
                return '1';
            }
            return '0';},
            Data::Char(val) => *val,
            Data::List(val, _) =>{dict.get_pntr(Pntr::Location(LocationPntr::new(val.line, 0))).to_char(dict)
            },
            Data::Num(val)=>(val % 255) as u8 as char,
            Data::Int(val)=>(val % 255) as u8 as char,
            Data::Unsigned(val)=>(val % 255) as u8 as char,
            Data::Double(val) => (val % 255.0) as u8 as char,
            _=>' ',
        }
    }
    pub fn to_ls(&self, dict:&mut Dict) -> Vec<Data>{
        match self{
            Data::Bool(val)=>vec![Data::Bool(*val)],
            Data::Char(val) =>vec![Data::Char(*val)],
            Data::List(val, _) =>dict.get_pntr_vec(val.clone()),
            Data::Num(val)=>vec![Data::Num(*val)],
            Data::Int(val)=>vec![Data::Int(*val)],
            Data::Unsigned(val)=>vec![Data::Unsigned(*val)],
            Data::Double(val) => vec![Data::Double(*val)],
            Data::Abstr(val,void) => vec![Data::Abstr((*val).clone(), *void)],
            Data::Func(val) => vec![Data::Func(val.clone())],
            _=>vec![Data::Null],
        }
    }
    pub fn to_str(&self, dict:&mut Dict) -> String{
        match self{
            Data::List(val, typ)=>{
                let mut result = "".to_string();
                let is_str = typ == &Type::Char;
                if !is_str{
                    result.push('[');
                    debug(format!("{:?} is not a string!", dict.get_pntr_vec(*val)), 2);
                }
                for line in dict.get_pntr_vec(*val){
                    result.push_str(&line.to_str(dict));
                    if !is_str{
                        result.push(',');
                    }
                }
                if !is_str{
                    result.pop();
                    result.push(']');
                }
                return result;
            },
            Data::Char(val)=>{
                return val.to_string();
            }
            _=>{
                return self.format(dict);
            }
        }
    }
    pub fn from_str(&self, var_type:Type, dict:&mut Dict)-> Data{
        let mut input = self.to_str(dict);
        input = string::remove_quotes(input);
        match var_type.clone(){
            Type::Bool=>{
                if input == "true"{
                    return Data::Bool(true);
                }
                if input == "false"{
                    return Data::Bool(false);
                }
            },
            Type::Num=>{
                if let Ok(val) = input.parse::<i128>(){
                    return Data::Num(val);
                }
            },
            Type::Int=>{
                if let Ok(val) = input.parse::<i32>(){
                    return Data::Int(val);
                }
            },
            Type::List(val)=>{
                let temp = input.clone().into_bytes();
                if temp[0] == ('[' as u8) || temp[0] == ('\"' as u8){
                    input.pop();
                    input.remove(0);
                    let parsed = string::split_parse(&mut input.clone().into_bytes(), &mut input);
                    let mut result:Vec<Data> = vec![];
                    for line in parsed{
                        let mut res:Vec<Data> = vec![];
                        for ch in line.chars(){
                            res.push(Data::Char(ch));
                        }
                        let respntr = dict.bind_pntr_temp(res);
                        result.push(Data::List(respntr, (*val).clone()).from_str((*val).clone(), dict));
                    }
                    let pntr = dict.bind_pntr_temp(result);
                    return Data::List(pntr, var_type);
                } else {
                    let mut res:Vec<Data> = vec![];
                    for ch in input.chars(){
                        res.push(Data::Char(ch));
                    }
                    let respntr = dict.bind_pntr_temp(res);
                    let temp = Data::List(respntr, (*val).clone()).from_str((*val).clone(), dict);
                    let pntr = dict.bind_pntr_temp(vec![temp]);
                    return Data::List(pntr, var_type);
                }
            },
            Type::Double=>{
                if let Ok(val) = input.parse::<f64>(){
                    return Data::Double(val);
                }
            },
            Type::Unsigned=>{
                if let Ok(val) = input.parse::<usize>(){
                    return Data::Unsigned(val);
                }
            },
            Type::Func=>{
                panic!("Cannot convert strings to functions!");
            },
            Type::Any=>{return self.clone();},
            Type::Null=>{return self.clone();},
            Type::Abstr=>{
                let handled = format::handle(vec![input]);
                let one_line = handled.len() == 1;
                return Data::Abstr(convert::convert_block(handled, dict), one_line);
            },
            Type::Char=>{
                return Data::Char(input.clone().chars().next().unwrap());
            },
            _=>{
                panic!("Not implemented yet!");
            }
        }
        return Data::Null;
    }
}//Conversion
impl Data{
    pub fn add(&self, operating:&Data, dict:&mut Dict) -> Data{
        match self{
            Data::Bool(val) => Data::Bool(*val ^ operating.to_bool(dict)),
            Data::Char(val) => Data::Char(((*val as u8) + (operating.to_char(dict) as u8)) as char),
            Data::Double(val) => Data::Double(*val + operating.to_float(dict)),
            Data::Int(val) => Data::Int(*val + operating.to_int(dict)),
            Data::Num(val) => Data::Num(*val + operating.to_num(dict)),
            Data::Unsigned(val) => Data::Unsigned(*val + operating.to_unsigned(dict)),
            Data::List(_, _) => self.add_list(operating, dict),
            _=>panic!("This operation (+) is not implemented for the chosen types!")
        }
    }
    pub fn add_list(&self, operating:&Data, dict:&mut Dict) -> Data{
        if let Data::List(val1, val2) = self{
            let mut val1_loaded = dict.get_pntr_vec(*val1);
            for line in &mut val1_loaded{
                *line = line.add(operating, dict);
            }
            dict.set_pntr_vec(*val1, val1_loaded);
            return Data::List(*val1, val2.clone());
        } else {
            panic!("This function was called in the wrong spot!");
        }
    }
}//Addition
impl Data{
    pub fn sub(&self, operating:&Data, dict:&mut Dict) -> Data{
        match self{
            Data::Bool(val) => Data::Bool(*val && ! operating.to_bool(dict)),
            Data::Char(val) => Data::Char(((*val as u8) - (operating.to_char(dict) as u8)) as char),
            Data::Double(val) => Data::Double(*val * operating.to_float(dict)),
            Data::Int(val) => Data::Int(*val * operating.to_int(dict)),
            Data::Num(val) => Data::Num(*val * operating.to_num(dict)),
            Data::Unsigned(val) => Data::Unsigned(*val * operating.to_unsigned(dict)),
            Data::List(_, _) => self.sub_list(operating, dict),
            _=>panic!("This operation (+) is not implemented for the chosen types!")
        }
    }
    pub fn sub_list(&self, operating:&Data, dict:&mut Dict) -> Data{
        if let Data::List(val1, val2) = self{
            let mut val1_loaded = dict.get_pntr_vec(*val1);
            for line in &mut val1_loaded{
                *line = line.sub(operating, dict);
            }
            dict.set_pntr_vec(*val1, val1_loaded);
            return Data::List(*val1, val2.clone());
        } else {
            panic!("This function was called in the wrong spot!");
        }
    }

}//Subtraction
impl Data{
    pub fn mult(&self, operating:&Data, dict:&mut Dict) -> Data{
        match self{
            Data::Bool(val) => Data::Bool(*val ^ operating.to_bool(dict)),
            Data::Double(val) => Data::Double(*val + operating.to_float(dict)),
            Data::Int(val) => Data::Int(*val + operating.to_int(dict)),
            Data::Num(val) => Data::Num(*val + operating.to_num(dict)),
            Data::Unsigned(val) => Data::Unsigned(*val + operating.to_unsigned(dict)),
            Data::List(_, _) => self.mult_list(operating, dict),
            _=>panic!("This operation (+) is not implemented for these types!")
        }
    }
    pub fn mult_list(&self, operating:&Data, dict:&mut Dict) -> Data{
        if let Data::List(val1, val2) = self{
            let mut val1_loaded = dict.get_pntr_vec(*val1);
            for line in &mut val1_loaded{
                *line = line.mult(operating, dict);
            }
            dict.set_pntr_vec(*val1, val1_loaded);
            return Data::List(*val1, val2.clone());
        } else {
            panic!("This function was called in the wrong spot!");
        }
    }
}//Multiplication
impl Data{
    pub fn div(&self, operating:&Data, dict:&mut Dict) -> Data{
        match self{
            Data::Bool(val) => Data::Bool(*val && operating.to_bool(dict)),
            Data::Double(val) => Data::Double(*val / operating.to_float(dict)),
            Data::Int(val) => Data::Int(*val / operating.to_int(dict)),
            Data::Num(val) => Data::Num(*val / operating.to_num(dict)),
            Data::Unsigned(val) => Data::Unsigned(*val / operating.to_unsigned(dict)),
            Data::List(_, _) => self.div_list(operating, dict),
            _=>panic!("This operation (+) is not implemented for the chosen types!")
        }
    }
    pub fn div_list(&self, operating:&Data, dict:&mut Dict) -> Data{
        if let Data::List(val1, val2) = self{
            let mut val1_loaded = dict.get_pntr_vec(*val1);
            for line in &mut val1_loaded{
                *line = line.div(operating, dict);
            }
            dict.set_pntr_vec(*val1, val1_loaded);
            return Data::List(*val1, val2.clone());
        } else {
            panic!("This function was called in the wrong spot!");
        }
    }

}//Division
impl Data{
    pub fn exp(&self, operating:&Data, dict:&mut Dict) -> Data{
        match self{
            Data::Bool(val) => Data::Bool(*val ^ operating.to_bool(dict)),
            Data::Double(val) => Data::Double(val.powf(operating.to_float(dict))),
            Data::Int(val) => Data::Int(val.pow(operating.to_unsigned(dict).try_into().unwrap())),
            Data::Num(val) => Data::Num(val.pow(operating.to_unsigned(dict).try_into().unwrap())),
            Data::Unsigned(val) => Data::Unsigned(val.pow(operating.to_unsigned(dict).try_into().unwrap())),//FIXME this shouldn't be here
            Data::List(_, _) => self.exp_list(operating, dict),
            _=>panic!("This operation (+) is not implemented for the chosen types!")
        }
    }
    pub fn exp_list(&self, operating:&Data, dict:&mut Dict) -> Data{
        if let Data::List(val1, val2) = self{
            let mut val1_loaded = dict.get_pntr_vec(*val1);
            for line in &mut val1_loaded{
                *line = line.exp(operating, dict);
            }
            dict.set_pntr_vec(*val1, val1_loaded);
            return Data::List(*val1, val2.clone());
        } else {
            panic!("This function was called in the wrong spot!");
        }
    }

}//Exponents
impl Data{
    pub fn modulus(&self, operating:&Data, dict:&mut Dict) -> Data{
        match self{
            Data::Double(val) => Data::Double(*val % operating.to_float(dict)),
            Data::Int(val) => Data::Int(*val % operating.to_int(dict)),
            Data::Num(val) => Data::Num(*val % operating.to_num(dict)),
            Data::Unsigned(val) => Data::Unsigned(*val % operating.to_unsigned(dict)),
            Data::List(_, _) => self.mod_list(operating, dict),
            _=>panic!("This operation (+) is not implemented for the chosen types!")
        }
    }
    pub fn mod_list(&self, operating:&Data, dict:&mut Dict) -> Data{
        if let Data::List(val1, val2) = self{
            let mut val1_loaded = dict.get_pntr_vec(*val1);
            for line in &mut val1_loaded{
                *line = line.modulus(operating, dict);
            }
            dict.set_pntr_vec(*val1, val1_loaded);
            return Data::List(*val1, val2.clone());
        } else {
            panic!("This function was called in the wrong spot!");
        }
    }
}//Modulus
impl Data{
    pub fn append(&self, operating:&Data, dict:&mut Dict) -> Data{
        let mut typ = self.to_type();
        if let Type::List(val) = &typ{
            typ = *val.clone();
        }
        let list1:Vec<Data> = self.to_ls(dict).iter().map(|x| x.to(typ.clone(), dict)).collect();
        let list2:Vec<Data> = operating.to_ls(dict).iter().map(|x| x.to(typ.clone(), dict)).collect();
        let full_list = vec![list1, list2].concat();
        return Data::List(dict.bind_pntr_temp(full_list), typ);
    }
    pub fn str_append(&self, operating:&Data, dict:&mut Dict) -> Data{
        let mut val = self.to_str(dict);
        let val2 = operating.to_str(dict);
        val.push_str(&val2);
        let chars:Vec<Data> = val.chars().map(|x| Data::Char(x)).collect();
        return Data::List(dict.bind_pntr_temp(chars), Type::List(Box::new(Type::Char)));
    }
}//Appending (++ (lists) and \+ (strs))
impl Data{
    pub fn compare(&self, operating:&Data, dict:&mut Dict, logic:fn(data1:&Data, data2:&Data) -> Data, recurse:fn(data1:&Data, data2:&Data, dict:&mut Dict, logic:fn(data1:&Data, data2:&Data)-> Data) -> Data) -> Data{
        if let Data::Bool(val) = self.compare_null(&operating, dict, logic){
            return Data::Bool(val);
        }
        if let Data::Bool(val) = self.compare_ls(&operating, dict, logic, recurse){
            return Data::Bool(val);
        }
        if let Data::Bool(val) = self.compare_double(&operating, dict, logic){
            return Data::Bool(val);
        }
        if let Data::Bool(val) = self.compare_num(&operating, dict, logic){
            return Data::Bool(val);
        }
        return Data::Null;
    }
    pub fn compare_null(&self, operating:&Data, _:&mut Dict, logic:fn(data1:&Data, data2:&Data)-> Data) -> Data{
        if let Data::Null = self{
            return logic(&Data::Null, operating);
        }
        if let Data::Null = operating{
            return logic(&Data::Null, self);
        }
        return Data::Null;
    }
    pub fn compare_ls(&self, operating:&Data, dict:&mut Dict, logic:fn(data1:&Data, data2:&Data) -> Data, recurse:fn(data1:&Data, data2:&Data, dict:&mut Dict, logic:fn(data1:&Data, data2:&Data)-> Data) -> Data) -> Data{
        if let Data::List(val1, typ1) = operating{
            if let Data::List(val2, typ2) = self{
                if dict.len(*val1) != dict.len(*val2){
                    return recurse(&Data::Unsigned(dict.len(*val1)), &Data::Unsigned(dict.len(*val2)), dict, logic);
                }
                else{
                    return recurse(&Data::List(val1.clone(), typ1.clone()), &Data::List(val2.clone(), typ2.clone()), dict, logic);
                }
            }
            else{
                let mut val2:Vec<Data> = vec![];
                for _ in 0..dict.len(*val1){
                    val2.push(self.clone());
                }
                return recurse(&Data::List(val1.clone(), typ1.clone()), &Data::List(dict.bind_pntr_temp(val2), self.to_type()), dict, logic);
            }
        }
        if let Data::List(val1, typ1) = self{
            if let Data::List(val2, typ2) = operating{
                if dict.len(*val1) != dict.len(*val2){
                    return recurse(&Data::Unsigned(dict.len(*val2)), &Data::Unsigned(dict.len(*val1)), dict, logic);
                }
                else{
                    return recurse(&Data::List(val2.clone(), typ2.clone()), &Data::List(val1.clone(), typ1.clone()), dict, logic);
                }
            }
            else{
                let mut val2:Vec<Data> = vec![];
                for _ in 0..dict.len(*val1){
                    val2.push(operating.clone());
                }
                return recurse(&Data::List(dict.bind_pntr_temp(val2), operating.to_type()), &Data::List(val1.clone(), typ1.clone()), dict, logic);
            }
        }
        return Data::Null;
    }
    pub fn compare_double(&self, operating:&Data, dict:&mut Dict, logic:fn(data1:&Data, data2:&Data) -> Data) -> Data{
        if let Data::Double(val1) = self{
            let val2 = operating.to_float(dict);
            return logic(&Data::Double(*val1), &Data::Double(val2));
        }
        if let Data::Double(val1) = operating{
            let val2 = self.to_float(dict);
            return logic(&Data::Double(val2), &Data::Double(*val1));
        }
        return Data::Null;
    }
    pub fn compare_num(&self, operating:&Data, dict: &mut Dict, logic:fn(data1:&Data, data2:&Data) -> Data) -> Data{
        let val1 = self.to_num(dict);
        let val2 = operating.to_num(dict);
        return logic(&Data::Num(val1), &Data::Num(val2));
    }
}//Comparison part 1
pub mod compare{
    use crate::dict::storage::Data;
    use crate::dict::*;
    pub fn recurse(data1:&Data, data2:&Data, dict:&mut Dict, logic:fn(data1:&Data, data2:&Data)-> Data) -> Data{//Recurse is only equipped to handle 4 types:
        match data1{//num, double, list, null. EVERYTHING ELSE GETS CONVERTED TO NUM IN 'COMPARE', EVEN BOOL. 
            Data::List(val1, _)=>{//If a list is detected...
                if let Data::List(val2, _) = data2{
                    for i in 0..dict.len(*val1){
                        if let Data::Bool(val) = dict.get_pntr_vec(*val1)[i].compare(&(dict.get_pntr_vec(*val2)[i]), dict, logic, recurse){//If we get a bool from this...
                            if val{//If that bool is true...
                                return Data::Bool(true);//Return true!
                            }
                        }
                    }
                    return Data::Bool(false);//If that loop finishes w/o a true value, return false
                }
                else{
                    panic!("You shouldn't see this message!");//We shouldn't get this. 
                }
            }
            _=>logic(data1, data2),//Otherwise, just go to the 'logic' function, which now only has to handle 3 types. 
        }
    }
    /*pub fn ex_recurse(data1:&Data, data2:&Data, logic:fn(data1:&Data, data2:&Data)-> Data) -> Data{//Recurse is only equipped to handle 4 types:
        match data1{//num, double, list, null. EVERYTHING ELSE GETS CONVERTED TO NUM IN 'COMPARE', EVEN BOOL. 
            Data::List(val1)=>{//If a list is detected...
                if let Data::List(val2) = data2{
                    for i in 0..val1.len(){
                        if let Data::Bool(val) = val1[i].compare(&*(val2[i]), logic, ex_recurse){//If we get a bool from this...
                            if !val{//If that bool is false...
                                return Data::Bool(false);//Return false!
                            }
                        }
                    }
                    return Data::Bool(true);//If that loop finishes w/o a false value, return true
                }
                else{
                    panic!("You shouldn't see this message!");//We shouldn't get this. 
                }
            }
            _=>logic(data1, data2),//Otherwise, just go to the 'logic' function, which now only has to handle 3 types. 
        }
    }//Just like "recurse", except, for lists, it's true only if it's true for all, instead of if it's tru for just 1
    */
    pub fn greater(data1:&Data, data2:&Data) -> Data{
        match data1{
            Data::Null => {if let Data::Null = data2{
                return Data::Bool(true);}//If we're comparing two nulls, true. 
                return Data::Bool(false);},//Otherwise, false. 
            Data::Double(val1)=>{
                if let Data::Double(val2) = data2{
                    return Data::Bool(val1 > val2);
                }
                else{
                    panic!("You shouldn't see this message!");//We shouldn't see this. The compare function condenses the values to 4 different types. 
                }
            }
            Data::Num(val1)=>{
                if let Data::Num(val2) = data2{
                    return Data::Bool(val1 > val2);
                }
                else{
                    panic!("You shouldn't see this message!");
                }
            }
            _=>{panic!("You shouldn't see this message!");}//You shouldn't have another type. If you do... AAAAAAAAAA! I'm not paid enough for this. 
        }
    }
    pub fn equals(data1:&Data, data2:&Data) -> Data{
        match data1{
            Data::Null => {if let Data::Null = data2{
                return Data::Bool(true);}//If we're comparing two nulls, true. 
                return Data::Bool(false);},//Otherwise, false. 
            Data::Double(val1)=>{
                if let Data::Double(val2) = data2{
                    return Data::Bool(val1 == val2);
                }
                else{
                    panic!("You shouldn't see this message!");//We shouldn't see this. The compare function condenses the values to 4 different types. 
                }
            }
            Data::Num(val1)=>{
                if let Data::Num(val2) = data2{
                    return Data::Bool(val1 == val2);
                }
                else{
                    panic!("You shouldn't see this message!");
                }
            }
            _=>{panic!("You shouldn't see this message!");}//You shouldn't have another type. If you do... AAAAAAAAAA! I'm not paid enough for this. 
        }
    }
    pub fn less(data1:&Data, data2:&Data) -> Data{
        match data1{
            Data::Null => {if let Data::Null = data2{
                return Data::Bool(true);}//If we're comparing two nulls, true. 
                return Data::Bool(false);},//Otherwise, false. 
            Data::Double(val1)=>{
                if let Data::Double(val2) = data2{
                    return Data::Bool(val1 < val2);
                }
                else{
                    panic!("You shouldn't see this message!");//We shouldn't see this. The compare function condenses the values to 4 different types. 
                }
            }
            Data::Num(val1)=>{
                if let Data::Num(val2) = data2{
                    return Data::Bool(val1 < val2);
                }
                else{
                    panic!("You shouldn't see this message!");
                }
            }
            _=>{panic!("You shouldn't see this message!");}//You shouldn't have another type. If you do... AAAAAAAAAA! I'm not paid enough for this. 
        }
    }
    pub fn geq(data1:&Data, data2:&Data) -> Data{
        match data1{
            Data::Null => {if let Data::Null = data2{
                return Data::Bool(true);}//If we're comparing two nulls, true. 
                return Data::Bool(false);},//Otherwise, false. 
            Data::Double(val1)=>{
                if let Data::Double(val2) = data2{
                    return Data::Bool(val1 >= val2);
                }
                else{
                    panic!("You shouldn't see this message!");//We shouldn't see this. The compare function condenses the values to 4 different types. 
                }
            }
            Data::Num(val1)=>{
                if let Data::Num(val2) = data2{
                    return Data::Bool(val1 >= val2);
                }
                else{
                    panic!("You shouldn't see this message!");
                }
            }
            _=>{panic!("You shouldn't see this message!");}//You shouldn't have another type. If you do... AAAAAAAAAA! I'm not paid enough for this. 
        }
    }
    pub fn leq(data1:&Data, data2:&Data) -> Data{
        match data1{
            Data::Null => {if let Data::Null = data2{
                return Data::Bool(true);}//If we're comparing two nulls, true. 
                return Data::Bool(false);},//Otherwise, false. 
            Data::Double(val1)=>{
                if let Data::Double(val2) = data2{
                    return Data::Bool(val1 <= val2);
                }
                else{
                    panic!("You shouldn't see this message!");//We shouldn't see this. The compare function condenses the values to 4 different types. 
                }
            }
            Data::Num(val1)=>{
                if let Data::Num(val2) = data2{
                    return Data::Bool(val1 <= val2);
                }
                else{
                    panic!("You shouldn't see this message!");
                }
            }
            _=>{panic!("You shouldn't see this message!");}//You shouldn't have another type. If you do... AAAAAAAAAA! I'm not paid enough for this. 
        }
    }
    pub fn neq(data1:&Data, data2:&Data) -> Data{
        match data1{
            Data::Null => {if let Data::Null = data2{
                return Data::Bool(false);}//If we're comparing two nulls, true. 
                return Data::Bool(true);},//Otherwise, false. 
            Data::Double(val1)=>{
                if let Data::Double(val2) = data2{
                    return Data::Bool(val1 != val2);
                }
                else{
                    panic!("You shouldn't see this message!");//We shouldn't see this. The compare function condenses the values to 4 different types. 
                }
            }
            Data::Num(val1)=>{
                if let Data::Num(val2) = data2{
                    return Data::Bool(val1 != val2);
                }
                else{
                    panic!("You shouldn't see this message!");
                }
            }
            _=>{panic!("You shouldn't see this message!");}//You shouldn't have another type. If you do... AAAAAAAAAA! I'm not paid enough for this. 
        }
    }
}//Comparison part 2
pub fn add(input:Vec<Data>, dict:&mut Dict) -> Vec<Code>{
    let operator = &input[1];
    let result = input[0].add(operator, dict);
    //debug(&format!("Adding result: {:?}", result));
    vec![Code::Object(Object::Literal(result))]
}
pub fn subtract(input:Vec<Data>, dict:&mut Dict) -> Vec<Code>{
    let operator = &input[1];
    let result = input[0].sub(operator, dict);
    //debug(&format!("Adding result: {:?}", result));
    vec![Code::Object(Object::Literal(result))]
}
pub fn multiply(input:Vec<Data>, dict:&mut Dict) -> Vec<Code>{
    let operator = &input[1];
    let result = input[0].mult(operator, dict);
    //debug(&format!("Adding result: {:?}", result));
    vec![Code::Object(Object::Literal(result))]
}
pub fn divide(input:Vec<Data>, dict:&mut Dict) -> Vec<Code>{
    let operator = &input[1];
    let result = input[0].div(operator, dict);
    //debug(&format!("Adding result: {:?}", result));
    vec![Code::Object(Object::Literal(result))]
}
pub fn append(input:Vec<Data>, dict:&mut Dict) -> Vec<Code>{
    let operator = &input[1];
    let result = input[0].append(operator, dict);
    //debug(&format!("Adding result: {:?}", result));
    vec![Code::Object(Object::Literal(result))]
}
pub fn append_str(input:Vec<Data>, dict:&mut Dict) -> Vec<Code>{
    let operator = &input[1];
    let result = input[0].str_append(operator, dict);
    //debug(&format!("Adding result: {:?}", result));
    vec![Code::Object(Object::Literal(result))]
}
pub fn exp(input:Vec<Data>, dict:&mut Dict) -> Vec<Code>{
    let operator = &input[1];
    let result = input[0].exp(operator, dict);
    //debug(&format!("Adding result: {:?}", result));
    vec![Code::Object(Object::Literal(result))]
}
pub fn mod_(input:Vec<Data>, dict:&mut Dict) -> Vec<Code>{
    let operator = &input[1];
    let result = input[0].modulus(operator, dict);
    //debug(&format!("Adding result: {:?}", result));
    vec![Code::Object(Object::Literal(result))]
}

pub fn not(input:Vec<Data>, dict:&mut Dict) -> Vec<Code>{
    let result = Data::Bool(input[0].not(dict));
    return vec![Code::Object(Object::Literal(result))];
}
pub fn and(input:Vec<Data>, dict:&mut Dict) -> Vec<Code>{
    let operator = input[1].clone();
    let result = input[0].and(&operator, dict);
    vec![Code::Object(Object::Literal(Data::Bool(result)))]
}
pub fn or(input:Vec<Data>, dict:&mut Dict) -> Vec<Code>{
    let operator = input[1].clone();
    let result = input[0].or(&operator, dict);
    vec![Code::Object(Object::Literal(Data::Bool(result)))]
}
pub fn equals(input:Vec<Data>, dict:&mut Dict) -> Vec<Code>{
    let temp = input[0].compare(&input[1], dict, compare::equals, compare::recurse);
    return vec![Code::Object(Object::Literal(temp))];
}
pub fn neq(input:Vec<Data>, dict:&mut Dict) -> Vec<Code>{
    let temp = input[0].compare(&input[1], dict, compare::neq, compare::recurse);
    return vec![Code::Object(Object::Literal(temp))];
}
pub fn geq(input:Vec<Data>, dict:&mut Dict) -> Vec<Code>{
    let temp = input[0].compare(&input[1], dict, compare::geq, compare::recurse);
    return vec![Code::Object(Object::Literal(temp))];
}
pub fn leq(input:Vec<Data>, dict:&mut Dict) -> Vec<Code>{
    let temp = input[0].compare(&input[1], dict, compare::leq, compare::recurse);
    return vec![Code::Object(Object::Literal(temp))];
}
pub fn greater(input:Vec<Data>, dict:&mut Dict) -> Vec<Code>{
    let temp = input[0].compare(&input[1], dict, compare::greater, compare::recurse);
    return vec![Code::Object(Object::Literal(temp))];
}
pub fn less(input:Vec<Data>, dict:&mut Dict) -> Vec<Code>{
    let temp = input[0].compare(&input[1], dict, compare::less, compare::recurse);
    return vec![Code::Object(Object::Literal(temp))];
}
pub fn if_parse(in1:&mut Vec<Code>, in2:&mut Vec<Code>, dict:&mut Dict) -> Option<Data>{
    if parse::evaluate(in1.clone(), dict).to_bool(dict){//We turn the input part 1 into a boolean. If it's true...
        let x = parse::parse(in2.clone(), dict);//We parse it out. 
        dict.set("=else", Data::Bool(false));//We then set the variable "=else" to false, so that no "else" statements are done. 
        return x;//We return the parsed value. 
    }
    dict.set("=else", Data::Bool(true));//Otherwise (no else needed because if it happens, it'll return) we set "=else" to true to "else" statements are done. 
    return None;//Nothing is returned. 
}
pub fn while_parse(in1:&mut Vec<Code>, in2:&mut Vec<Code>, dict:&mut Dict) -> Option<Data>{
    dict.set("=break", Data::Bool(false));//At the beginning, we make sure that we won't break. 
    while !dict.get("=break").to_bool(dict){//While we aren't breaking, and the evaluation of in1 returns true...
        dict.new_scope();
        if !parse::evaluate(in1.clone(), dict).to_bool(dict){dict.drop_scope();break;};
        dict.set("=break", Data::Bool(false));//Just in case...
        if let Some(val) = parse::parse(in2.clone(), dict){//If we find a value, we return it. 
            dict.drop_scope();
            return Some(val);
        }
        dict.drop_scope();//drops scope
    }
    return None;
}
pub fn else_parse(in1:&mut Vec<Code>, in2:&mut Vec<Code>, dict:&mut Dict) -> Option<Data>{
    if !dict.get("=else").to_bool(dict){
        dict.set("else", Data::Bool(true));
        return None;
    }
    if in1.len() == 0 || parse::evaluate(in1.clone(), dict).to_bool(dict){
        dict.set("=else", Data::Bool(false));
        return parse::parse(in2.clone(), dict);
    }
    
    dict.set("=else", Data::Bool(true));
    return None;
}