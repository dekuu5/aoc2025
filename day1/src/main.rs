use std::fs;

fn main() {
    
    
    let input = fs::read_to_string("./day1/test.txt").unwrap();
    let out = process_input(input);
    println!("password {}", p1(out.clone()));
    println!("clicks {}", p2(out));
    println!("{}", (100i32).rem_euclid(100))

    
}


#[derive(Debug, PartialEq, Clone)]
enum Action {
    Left(i32),
    Right(i32)
}

const SIZE: i32 = 100;

fn p1(actions : Vec<Action>) -> i32 {
    let mut pos = 50 ;
    let mut password = 0;
    for action in actions {
        match action {
            Action::Left(c) => {
                pos = (pos - c).rem_euclid(SIZE);
                    
                
            },
            Action::Right(c) => {
                pos = (pos+ c).rem_euclid(SIZE);
                
            },
        }
        println!("{pos}");
        if pos == 0 {
            password +=1;
        }
    }
    password
}

fn p2(actions : Vec<Action>) -> i32 {
    let mut pos = 50;
    let mut clicks = 0;
    for action in actions {
        match action {
            Action::Left(c) => {
                clicks += c /SIZE;
                let rem = c % SIZE;
                if pos > 0 && (pos - rem) <= 0 {
                    clicks += 1;
                }
                pos = (pos- rem).rem_euclid(SIZE);
                
            },
            Action::Right(c) => {
                pos += c;
                clicks += pos / 100;

                
                pos = pos.rem_euclid(SIZE);
                
            },
        }
        println!("{pos}");
        
    }
    clicks
}


fn process_input(input:String) -> Vec<Action> {
    let mut res = vec![];
    for act in input.split("\n") {
        res.push(match act.chars().nth(0) {
            Some('R') => Action::Right(act[1..].parse().unwrap()),
            Some('L') => Action::Left(act[1..].parse().unwrap()),
            _ => unreachable!()
        });
    }
    
    res
}

