
const DEBUG:[bool;10] = [false, false, false, false, false, false, false, false, false, false];
const OVERRIDE_TRUE:bool = false;
const OVERRIDE_FALSE:bool = false;
pub fn debug(input:String, channel:usize){
    if (DEBUG[channel] || OVERRIDE_TRUE) && !OVERRIDE_FALSE{
        println!("{}", input);
    }
}