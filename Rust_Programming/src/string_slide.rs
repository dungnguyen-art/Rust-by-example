fn main(){
   let split = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.".split(" ");
   let str_cnt = "is";
   let mut _cnt:i32 = 0;
    for s in split {
        let s = &s.replace(".","");
        if s == str_cnt {
            _cnt += 1;
        }
    }
    println!("{}",_cnt);
}