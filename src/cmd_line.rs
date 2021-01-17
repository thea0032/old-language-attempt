use std::io;
use std::io::stdout;
use std::io::Write;
use crate::debug::debug;
#[allow(dead_code)]
const RETURNS:usize = 2;
/*use std::thread;
use std::time;
use math;*/
/*pub fn sleep(m_secs:u64){
    let millis = time::Duration::from_millis(m_secs);
    let now = time::Instant::now();
    thread::sleep(millis);
    assert!(now.elapsed() >= millis);   
}//Sleeps for m_secs milliseconds. */
#[allow(dead_code)]
pub fn get_string(input:&str) -> String{
    let mut source = String::new();//Creates a variable to put the string to. 
    print!("{}", input);
    let _ = stdout().flush();
    io::stdin().read_line(&mut source).expect("Invalid input!");//Adds the line to the string.  
    debug(format!("We used get_string, and got {:?}", source), 0);
    for _ in 0..RETURNS{
        source.pop();
    }
    source
}