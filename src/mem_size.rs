fn main() {
    let result = factorial(5);
    println!("Factorial of 5 is: {}", result);

    #[allow(overflowing_literals)]
    let x = { 255 as i8 };

    // You could solve this by using exactly the same expression as above,
    // but that would defeat the purpose of the exercise. Instead, use a genuine
    // `i8` value that is equivalent to `255` when converted to `u8`.
    let y: i8 = x as i8;
    println!("x: {}, y: {}", x, y);

    let size_of_string = std::mem::size_of::<String>();
    println!("Size of String: {}", size_of_string);
    let size_of_vec = std::mem::size_of::<Vec<u8>>();
    println!("Size of Vec<u8>: {}", size_of_vec);
    let size_of_u8 = std::mem::size_of::<u8>();
    println!("Size of u8: {}", size_of_u8);
    let size_of_bool = std::mem::size_of::<bool>();
    println!("Size of bool: {}", size_of_bool);

    let size_of_u32 = std::mem::size_of::<u32>();
    println!("Size of u32: {}", size_of_u32);
    let size_of_my_struct = std::mem::size_of::<MyStruct>();
    println!("Size of MyStruct: {}", size_of_my_struct);

    let size_of_my_second_struct = std::mem::size_of::<MySecondStruct>();
    println!("Size of MySecondStruct: {}", size_of_my_second_struct);

    // Mutable sizes
    // let size_of_string_mut = std::mem::size_of::<String>();

    // Reference sizes
    let size_of_string_ref = std::mem::size_of::<&String>();
    println!("Size of &String: {}", size_of_string_ref);
    let size_of_vec_ref = std::mem::size_of::<&Vec<u8>>();
    println!("Size of &Vec<u8>: {}", size_of_vec_ref);
    let size_of_u8_ref = std::mem::size_of::<&u8>();
    println!("Size of &u8: {}", size_of_u8_ref);
    let size_of_bool_ref = std::mem::size_of::<&bool>();
    println!("Size of &bool: {}", size_of_bool_ref);
    let size_of_u32_ref = std::mem::size_of::<&u32>();
    println!("Size of &u32: {}", size_of_u32_ref);
    let size_of_my_struct_ref = std::mem::size_of::<&MyStruct>();
    println!("Size of &MyStruct: {}", size_of_my_struct_ref);
    let size_of_my_second_struct_ref = std::mem::size_of::<&MySecondStruct>();
    println!("Size of &MySecondStruct: {}", size_of_my_second_struct_ref);

    // MUT reference sizes
    let size_of_string_mut_ref = std::mem::size_of::<&mut String>();
    println!("Size of &mut String: {}", size_of_string_mut_ref);
    let size_of_vec_mut_ref = std::mem::size_of::<&mut Vec<u8>>();
    println!("Size of &mut Vec<u8>: {}", size_of_vec_mut_ref);
    let size_of_u8_mut_ref = std::mem::size_of::<&mut u8>();
    println!("Size of &mut u8: {}", size_of_u8_mut_ref);
    let size_of_bool_mut_ref = std::mem::size_of::<&mut bool>();
    println!("Size of &mut bool: {}", size_of_bool_mut_ref);
    let size_of_u32_mut_ref = std::mem::size_of::<&mut u32>();
    println!("Size of &mut u32: {}", size_of_u32_mut_ref);
    let size_of_my_struct_mut_ref = std::mem::size_of::<&mut MyStruct>();
    println!("Size of &mut MyStruct: {}", size_of_my_struct_mut_ref);
    let size_of_my_second_struct_mut_ref = std::mem::size_of::<&mut MySecondStruct>();
    println!("Size of &mut MySecondStruct: {}", size_of_my_second_struct_mut_ref);

}

pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 0..n {
        result *= n - i;
    }

    result
}

struct MyStruct {
    field1: u32,
    field2: String,
}

struct MySecondStruct {
    field2: u32,
    field1: String,
  
    field5: String,
}