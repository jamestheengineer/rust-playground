fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest_num = &number_list[0];

    for number in &number_list {
        if number > largest_num {
            largest_num = number;
        }
    }

    println!("The largest number is {}", largest_num);

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let x = 5;              // ----------+-- 'b
                            //           |
    let r = &x;             // --+-- 'a  |
                            //   |       |
    println!("r: {}", r);   //   |       |
                            // --+       |
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}                           // ----------+

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}