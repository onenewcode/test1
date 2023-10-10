#[derive(Debug)]
struct Mycode {
    strcode: String,
}
trait Code {
    fn mydecode(&self)->String;
    fn myencode(&self)->String;
    
}
impl Code for Mycode{   
    fn mydecode(&self)->String{
        self.strcode.replace("111101", "11111")
    }

    fn myencode(&self)->String {
        self.strcode.replace("11111", "111101")
    }
}
impl Mycode {
    fn new(strcode: String) -> Mycode {
        // // 滑动窗口统计0，1字符的个数
        let substrings = strcode.trim().chars()
        // .windows(1)
        .filter(|&w| w == '0'|| w == '1' )
        .count();
        println!("{},{}",substrings,strcode.trim().len());
        // 去除空格
        if substrings!=strcode.trim().len(){
            panic!("字符流含有0，1外的字符程序终止")
        }
        
            Mycode {
                strcode,
            }
    }
}
use std::{io::stdin, str::Chars};
fn main() {
    println!("请输入需要编码的零一比特流");

    let mut your_name = String::new();

    stdin()
        .read_line(&mut your_name)
        .expect("Failed to real line!");

    let input=Mycode::new(your_name);
    println!("原本字符流{}",input.strcode);
    println!("加密后字符流{}",input.myencode());

    println!("解密后字符流{}",input.strcode);



}