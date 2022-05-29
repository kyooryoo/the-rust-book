fn main() {
    println!("Hello, flows!");

    let result = if 1 == 2 {
		"Can't be true!"
	} else {
		"Rust makes sense!"
	};
	println!("You know what? {}", result);

    let status = req_status();
    match status {
        200 => println!("Success!"),
        404 => println!("Not Found!"),
        other => {
            println!("Request failed with code: {}", other);
        }
    }

    let mut x = 5;
    loop {
        if x < 0 {
            break;
        }
        println!("{} more runs to go!", x);
        x -= 1;
    }

    let a = 10;
    let b = 4;
    let result = silly_sub(a,b);
    println!("{} minus {} is {}!", a, b, result);

    print!("Normal Range from 0 to 10: ");
    for i in 0..10 {
        print!("{},", i);
    }

    println!();
    print!("Inclusive Range from 0 to 10: ");
    for i in 0..=10 {
        print!("{},", i);
    }
}

fn req_status() -> u32 {
    200
}

fn silly_sub(a: i32, b: i32) -> i32 {
    let mut result = 0;
    'increment: loop {
        if result == a {
            let mut dec = b;
            'decrement: loop {
                if dec == 0 {
                    break 'increment;
                } else {
                    result -= 1;
                    dec -= 1;
                }
            }
        } else {
            result += 1;
        }
    }
    result
}
