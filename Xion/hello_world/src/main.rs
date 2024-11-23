fn main() {
    println!("Hello, world!");

    let mut a =10;
    a=15;
    let b=20;
    let c =30;
    println!("number is {}{}{}",a,b,c);

    let char='C';
    println!("Char is {}",char);

    let float_num=7.14;
    println!("Float is {}",float_num);

    const xy: i32 = 15;
    println!("const is {}",xy);

    let my_name = String::from("CHAITANYA");
    println!("My name {}",my_name);

    let mut my_vec: Vec<u64> = Vec::new();
    my_vec.push(245);

    println!("my vector/array is{:?}", my_vec);

    struct rectangle{
        len: u32, 
        wid: u32,
    };

    let my_rectangle: rectangle = rectangle{
        len:10,
        wid:15,
    };
    println!("length and width of rectangle is {} {}",my_rectangle.len, my_rectangle.wid);

    let x:u32=sum(10, 20);
    println!("sum is {}",x);



}

fn sum(a: u32, b:u32) -> u32 {
    let x: u32 =a+b;
    return x;
}
