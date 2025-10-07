fn main(){
loop_expression();
loop_return_value();
loop_label();
while_loop();
for_loop();
range();
range_inc();
}

fn loop_expression(){
    let mut counter = 0;

    loop{
        counter += 1;

        if counter > 10 {
            break;
        }else if counter % 2 == 0{
            continue;
        }

        println!("Counter : {}", counter);
    };
}

fn loop_return_value(){
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10{
            break counter * 2;
        }
    };
    println!("{}", result);
}

fn loop_label(){
    let mut number = 1;
    'outer: loop{
        let mut i =1;
        loop{
            if number > 10{
                break 'outer;
            };
            println!("{} x {} = {}", number , i, i*number);
            i +=1;
            if i > 10{
                break;
            }
        }
        number += 1;
    }
}
fn while_loop(){
    let mut counter = 0;
    while counter <= 10{
        if counter % 2 == 0 {
            println!("Counter : {}", counter);
        }
        counter += 1;
    }
}

fn for_loop(){
    let array: [&str; 5] = ["a", "b", "c","d","e"];

    for value in array{
        println!("Value : {}", value);
    }
}
fn range(){
    let array: [&str; 5] = ["a", "b", "c","d","e"];
    let range = 0..5;

    println!("Start ; {}", range.start);
    println!("Start ; {}", range.end);

    for i in range{
        println!("Value {}", array[i]);
    }
}
fn range_inc(){
    let array: [&str; 5] = ["a", "b", "c","d","e"];
    let range = 0..=4;

    println!("Start ; {}", range.start());
    println!("Start ; {}", range.end());

    for i in range{
        println!("Value {}", array[i]);
    }
}