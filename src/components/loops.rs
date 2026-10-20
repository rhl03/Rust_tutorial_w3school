pub fn loops() {
    let mut count = 1;

    loop {
        println!("Hello world!");
        if count == 3 {
            break;
        }

        count += 1;
    }
}
pub fn loop_with_value() {
    let mut count = 1;

    let result = loop {
        println!("Hello!");

        if count == 3 {
            break count;
        }

        count += 1;
    };

    println!("The loop stopped at: {}", result);
}

pub fn while_loop() {
    let mut count = 1;

    while count <= 5 {
        println!("Count: {}", count);
        count += 1;
    }
}

pub fn while_loop_break() {
    let mut num = 1;

    while num <= 10 {
        if num == 6 {
            break;
        }
        println!("Number: {}", num);
        num += 1;
    }
}

pub fn skip_value() {
    let mut num = 1;

    while num <= 10 {
        if num == 6 {
            num += 1;
            continue;
        }

        println!("Number: {}", num);
        num += 1;
    }
}

pub fn for_loop() {
    for i in 1..6 {
        println!("i is: {}", i);
    }
}

pub fn inclusive_loop() {
    for i in 1..=6 {
        println!("is is: {}", i);
    }
}

pub fn break_continue_loop() {
    for i in 1..=10 {
        if i == 3 {
            continue;
        }
        if i == 5 {
            break;
        }
        println!("i is: {}", i);
    }
}
