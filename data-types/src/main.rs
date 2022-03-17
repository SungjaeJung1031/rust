fn main() 
{
    let ai32_arr: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    let str_arr: [String; 3];
    str_arr = ["alice".to_string(), "atlas".to_string(), "lhc-b".to_string()];

    let ai32_arr_3 = ai32_arr[3];
    let str_arr_0 = str_arr[0].clone();
    // let str_arr_4 = str_arr[4].clone();      // incorrect array indexing

    println!("The first element of ai32_arr is: {}", ai32_arr_3);
    println!("The first element of str_arr is: {}", str_arr_0);
}