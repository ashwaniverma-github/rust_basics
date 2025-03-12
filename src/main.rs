
fn main(){   
    print!("{}",is_even(20));
    println!("Fibonacci(5)- {}" , fib(5));
    println!("{}",get_string("mihablo"));
    data();

    let rect = Shape::Rectangle(12.00, 2.00);
    println!("area of rectangle is - {:?}",area(rect));

    let circle = Shape::Circle(10.00);
    println!("area of circle is - {:?}",area(circle));

    vec1()
}

fn is_even(num1:i128)->bool{
    if num1%2==0{
        return true
    }else{
        return false
    }
    
}

fn fib(n:u32)->u32{
    if n==0 {
        0;
    } 
    let mut a = 0;
    let mut b = 1;

    for _ in 1..n{
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

fn get_string(str1:&str)->usize{
    str1.chars().count()
}

// Structs lets you structure data together ther are slighly closure to classes in rust

struct User {
    name:String,
    is_signed:bool,
    money_spent_1:u32,
    money_spent_2:u32,
}

impl User {
    fn total_money_spent(&self)->u32{
        self.money_spent_1 + self.money_spent_2
    }
}

fn data(){
    let user1 = User {
        name:String::from("juan"),
        is_signed:true,
        money_spent_1:500,
        money_spent_2:200
    };

    println!("{:?}",user1.name);
    println!("{:?}",user1.is_signed);
    println!("{:?}",user1.money_spent_1);

    println!("total money spent is  - {}" , user1.total_money_spent() )
}

// Enums  - let you enumerate over various types of value 

enum Shape {
    Rectangle(f64 , f64),
    Circle(f64),
    
}

fn area(shape:Shape) -> f64 {
    let x = match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
    };
    return x;
}

// Collections can contain multiple values the data they point stored in a heap.

// Vectors 

fn vec1(){
    let mut vec = Vec::new();
    vec.push(8);
    vec.push(1);
    vec.push(10);
    vec.push(9);
    println!("{:?}",vec); // OwnerShip Also goes the the even_vec use &vec to keep the current owner and pass it as reference
    even_vec(vec);
    
}

fn even_vec(vec: Vec<i32>){
    let mut vec2 = Vec::new();

    for value in vec {
        if value % 2 == 0 {
            vec2.push(value)
        }
    }
    println!("{:?}",vec2)
}