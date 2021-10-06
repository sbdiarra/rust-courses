fn main() {
    let mut v: Vec<i32> = Vec::new();
    let macroVec = vec![1, 2, 3, 4, 5];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &macroVec[2];
    println!("The third element is {}", third);

    match macroVec.get(2) {
        some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let does_not_exist = &macroVec[100];
    let does_not_exist = macroVec.get(100);

    for i in &v {
        println!("{}", i)
    }

    for i in &mut v {
        *i += 50;
    }

    // store different type with enum
    enum SpreadCell {}
}
