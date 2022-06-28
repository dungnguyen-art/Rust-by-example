fn main(){
    let arg_arr = vec![1,2,3,4,5,6,10,11];
    let sub_arr = vec![6,8,10];

    let mut check = true;
    for i in 0..3{
        if !arg_arr.contains(&sub_arr[i]) {
            check = false;
            println!("ahahah");
            break;
        }
    }

    if check {
        println!("YES");
    }
    else{
        println!("NO");
    }
}