
mod storage;
mod format;
mod string;
mod convert;
mod debug;
mod parse;
mod sto_skeleton;
mod operations;
mod raw_ops;
mod raw_ops2;
mod file;
mod builtin;
mod cmd_line;
mod dict;
mod modules;
mod format2;
mod input;
mod new_ops;
use crate::dict::Dict;
use crate::dict::storage::Data;
pub fn main(){
    loop{
        let mut file = file::read_new_file();
        let filename = file.remove(0);
        let format = format2::handle(file);
        debug::debug(format!("Formatted: {:?}", format), 3);
        let mut dict = dict::Dict::new(filename);
        dict.init();
        parse::parse_grid(format, &mut dict);
    }
}