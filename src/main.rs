fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // this is a reference to the 3rd element and an immutable 
    // borrow
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // this method returns an option type
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // this causes a panic because it does not return an option type
    // let _does_not_exist = &v[100];

    // this returns an option type
    let _does_not_exist = v.get(100);

    // can't do an immtuable borrow followed by a mutable
    //let first = &v[0];

    // mutable borrow
    v.push(6);

    // but you can do an immutable borrow here
    let first = &v[0];

    // there is no mutable borrow between the use of this immutable and
    // when it was assigned
    println!("The first element is: {}", first);

    // you can iterate through a vector's elements
    for i in &v {
        println!("{}", i);
    }
    
    // you can also iterate mutably to make changes to the vector's elements
    for i in &mut v {
        // note the deref
        *i += 50; 
    }

    // you can make a vector hold different types by using the variants of an
    // enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
