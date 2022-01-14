#[derive(PartialEq)]
enum size_enum {
    BIG,
    MEDIUM,
    SMALL
}

fn to_str(t: size_enum) -> String {
    if t == size_enum::BIG {
        return "BIG".to_string();
    } else if t == size_enum::MEDIUM {
        return "MEDIUM".to_string();
    } else {
        return "SMALL".to_string();
    }
}

fn main() {
    const MAX: usize = 100;
    let mut arr: [i64; MAX] = [0; MAX];

    let mut i: usize = 0;
    let mut ii: i64 = 0;
    while i < MAX {
        arr[i] = ii;
        i += 1;
        ii += 1;
    }

    let mut t: size_enum;
    for x in arr {        
        if x < 33 {
            t = size_enum::SMALL;
        } else if x < 66 {
            t = size_enum::MEDIUM;
        } else {
            t = size_enum::BIG;
        }
        
        println!("{} is {}", x, to_str(t));
    }
}
