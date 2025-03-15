use std::collections::HashMap;


fn main(){   
    print!("{}",is_even(20));
    println!("Fibonacci(5)- {}" , fib(5));
    println!("{}",get_string("mihablo"));
    data();

    let rect = Shape::Rectangle(12.00, 2.00);
    println!("area of rectangle is - {:?}",area(rect));

    let circle = Shape::Circle(10.00);
    println!("area of circle is - {:?}",area(circle));

    vec1();
    hashmap();

    let input_vec = vec![(String::from("Leo"),18) , (String::from("joshe"),21)];
    println!("{:?}",hashmap_val_group(input_vec));

    iter();
    mut_iter();
    into_iter();
    cons_adaptor();
    iter_adaptor();
    assignment();
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

// Structs lets you structure data together

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

    // 1- Vectors 

fn vec1(){
    let mut vec: Vec<i32> = Vec::new();
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

    // 2- Hashmap Stores key value pair in rust 

fn hashmap(){

    let mut data = HashMap::new();

    data.insert(String::from("Jose"), 10);
    data.insert(String::from("Joshua"), 18);

    let user_age = data.get("Jose");

    // println!("{}",user_age); 

    // above print statement gives Error cause the return type is Option<i32> it could return something or null so u will have to handle both,
    // Using match.

    match user_age {
        Some(age)=>println!(" age of Jose - {}",age),
        None=>println!("Empty value")
    }
    
}

fn hashmap_val_group(vec:Vec<(String,i32)>)->HashMap<String , i32>{
    let mut hash_map = HashMap::new();
    for (key,value) in vec{
        hash_map.insert(key, value);
    }
    return hash_map;
}

// Iterators 

// 1)  immutable iterator

fn iter(){
    let x = vec![1,2,3,4];
    let iter = x.iter(); // borrows value from x vector

    for nums in iter{
        println!("{}",nums)
    }
    println!("{:?}",x) // owner of the vector will still be x 
}

// 2) Mutable iterator

fn mut_iter(){
    let mut x = vec![1,2,3];

    let iter = x.iter_mut(); // it takes a mutable reference does not consumed the x vector.

    for nums in iter{
        *nums = *nums *2;
        println!("muted - {}",nums)
    }
    println!(" muted- {:?}",x); // owner the vector will still be x
}

// 3) into_iter - takes ownership 

fn into_iter(){
    let x = vec![3,2,1];

    let iter = x.into_iter(); // consumes the x vector 

    for nums in iter{
        println!("owned-{}",nums)
    }
    // print!("{:?}",x) will give error as x is nomore an owner

}

// Consuming adaptor - methods that consumes the iterator resulting in not able to use that iterator anymore

fn cons_adaptor(){
    let x  = vec![4,5,6];

    let iter = x.iter();
    
    let sum:i32 = iter.sum(); // consumes / takes ownership of iter

    println!("{}",sum);

    // for nums in iter{  // gives error as the sum method owns iterator now
    //     println!("{}",nums)
    // }


}

// iterator adaptor - Iterator adaptors like map are lazy and produce a new iterator for further chaining. However, they also take ownership of the original iterator—so you must use the new iterator, not the original.

fn iter_adaptor(){
    let x = vec![7,8,9];

    let iter = x.iter();

    let map_method = iter.map(|x| x+3);
    
    for nums in map_method{
        println!("{}",nums)
    }
}


// Write the logic to first filter all odd values then double each value and create new vector
fn assignment(){
    let v1 = vec![2,5,3,4,8,7];

    let res = v1.into_iter().filter(|x| *x%2 !=0 ).map(|x|x*2);
    let v2:Vec<i32> = res.collect(); // converts iterator to vector
    println!("{:?}",v2)

}

