fn print_scalar_types() {
    // Unsigned: pertenecera a los numeros naturales, al no importar el signo.
    // Signed: pertenece al dominio de los enteros, puede ser negativo.
    let my_unsigned_int: u32 = 2;
    let my_signed_int: i32 = -5;
    println!("Unsigned: {my_unsigned_int}, Signed: {my_signed_int}");

    // 32-bit float are mean to have a single precision float.
    // While 64-bit float are mean to have a double precision float.
    let single_float: f32 = 2.0;
    let double_float = 3.0;
    println!("Single precision: {single_float}, Double precision: {double_float}");

    // Booleans are as simple as in other langs.
    let my_bool: bool = true;
    println!("Is this a boolean? {my_bool}");

    // Characters are different from string literals.
    let my_emoji = '🤖';
    let my_char = 'z';
    println!("{my_emoji} My secret char {my_char}!");
}

fn print_compound_types() {
    // Tuple type: listas de longitud fija, cuyos tipos deben ser declarados.
    let tuple: (i32, f64, u8) = (-500, 6.4, 1);
    // Tuples can be destructured to extract it's values.
    let (x, y, z) = tuple;
    println!("Tuple contains the values: {x}, {y}, {z}");
    let first_tuple_value = tuple.0;
    println!("You can also access values as a list: {first_tuple_value}");

    // Array type: lista de longitud fija, cuyos tipos deben ser el mismo para cada elemento.
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    // 
    // If you want to generate a list of fixed length with a value you can do:
    // let generated_array: [3; 5];
    // let first_gen_array = generated_array[0];
    // println!("First generated value: {first_gen_array}");
    //
    let first_array_value = my_array[0];
    println!("First value on the array: {first_array_value}");
}

fn main() {
    print_scalar_types();
    print_compound_types();
}
