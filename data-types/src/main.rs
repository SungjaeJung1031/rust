fn main() 
{
    // array
    let ai32_arr: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    let str_arr: [String; 3];
    str_arr = ["alice".to_string(), "atlas".to_string(), "lhc-b".to_string()];

    let ai32_arr_3 = ai32_arr[3];
    let str_arr_0 = str_arr[0].clone();
    // let str_arr_4 = str_arr[4].clone();      // incorrect array indexing

    println!("The third element of ai32_arr is: {}", ai32_arr_3);
    println!("The first element of str_arr is: {}", str_arr_0);


    // tuple
    let short_tup = (100, 1.1);
    let (x, _y) = short_tup;
    let long_tup = (1u8, -2i16, 0.1f32, 'c', true);
    let tuple_of_tuples = (('a', 'b', 'c'), (1u8, 2u8), (0.1f32));

    println!("short_tup: {:?}", short_tup);
    println!("the first value of short_tup: {}", short_tup.0);
    println!("the second value of short_tup: {}", short_tup.1);
    println!("long_tup: {:?}", long_tup);
    println!("tuple_of_tuples: {:?}", tuple_of_tuples);
    println!("x: {}", x);

    let short_tup: (i32,f64) = (100, 1.1);
    println!("the first value of short_tup: {}", short_tup.0);
    println!("the sencond value of short_tup: {}", short_tup.1);
}