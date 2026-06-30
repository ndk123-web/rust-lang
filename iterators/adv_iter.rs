/**
    ⭐⭐⭐⭐⭐ map()
    ⭐⭐⭐⭐⭐ filter()
    ⭐⭐⭐⭐⭐ collect()
    ⭐⭐⭐⭐⭐ enumerate()
    ⭐⭐⭐⭐⭐ zip()

    ⭐⭐⭐⭐ fold()
    ⭐⭐⭐⭐ find()
    ⭐⭐⭐⭐ find_map()
    ⭐⭐⭐⭐ any()
    ⭐⭐⭐⭐ all()

    ⭐⭐⭐ chain()
    ⭐⭐⭐ flat_map()
    ⭐⭐⭐ peekable()
    ⭐⭐⭐ take()
    ⭐⭐⭐ skip()

    ⭐⭐⭐ by_ref()
    ⭐⭐⭐ inspect()
    ⭐⭐⭐ rev()

    ⭐⭐⭐⭐ Custom Iterator
*/

fn main() {

    //1. Map (Collect assign new memory in heap, put values and returns it)
    let v: Vec<i32> = vec![1,2,3,4,5];

    let res: Vec<i32> = v
                .iter()
                .map(|x| {
                    *x * 2
                })
                .collect();

    println!("{:?}", v);
    println!("{:?}", res);

    // 2. filter
    let v: Vec<i32> = vec![1,2,3,4,5,6];

    let res: Vec<_> = v 
                        .iter()
                        .filter(|x| *x % 2==0)
                        .collect();
    
    println!("{:?}", v);
    println!("{:?}", res);
    
    // 4. enumerate
    let v: Vec<i32> = vec![1,2,3,4,5,6];

    for (i, val) in v.iter().enumerate() {
        println!("{}-{}", i, val);
    }
    
    // 5. zip
    let names = vec!["Ndk", "Vsh", "Krt"];
    let ages = vec![21,22,22];
    
    for (name, age) in names.iter().zip(ages.iter()) {
        println!("{}-{}", name,age);
    }

    // 6. fold (accumulate)
    let v = vec![1,2,3,4,5];
    let res : _ = v
                    .iter()
                    .fold(0, |acc, x| { acc + x  });
    println!("Sum: {}", res);

    // 7. find (return Option)
    let v = vec![1,2,3,4,5];
    if let Some(val) = v.iter().find(|x| { **x == 2  }) {
        println!("Found-{}", val);
    }

    // 8. any
    let v = vec![1,2,3,4,5];
    if v.iter().any(|x| *x == 2) {
        println!("Found 2");
    }

    // 9. all
    if v.iter().all(|x| *x > 0) {
        println!("All > 0");
    }
    
}
