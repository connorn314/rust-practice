fn main() {
    let s: String = String::from("abcdefghijklmnopqrstuvwxyz");
    for char in s.bytes(){
        println!("{char}")
    }

    // let third: i32 = &v[2];
    // println!("The third element is {third}");

    // let third: Option<&i32> = v.get(2);
    // match third {
    //     Some(third) => println!("The third element is {third}"),
    //     None => println!("There is no third element."),
    // }
    let v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    // v.push(6);

    println!("The first element is: {first}");
    // let does_not_exist = &v[100];
    let does_not_exist = v.get(3);
    match does_not_exist {
        Some(num) => println!("The 100th element is {num}"),
        None => println!("There is no 100th element."),
    }
}
