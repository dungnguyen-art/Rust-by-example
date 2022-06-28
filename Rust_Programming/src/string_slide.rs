use std::io;

fn main(){
   let split = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.".split(" ");
   let mut str_cnt = String::new();
   io::stdin().read_line(&mut str_cnt).expect("could read file");
   let new_str_cnt = str_cnt.to_string();
   let new_str_cnt = new_str_cnt.trim();

   let mut _cnt:i32 = 0;
    for s in split {
        let new_string = String::from(&s.replace(".",""));
        if new_string == new_str_cnt {
           _cnt += 1;
       }
    }
    println!("{}",_cnt);
}