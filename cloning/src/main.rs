#[derive(Debug, Clone)]
pub struct TestStruct {
    pub value1: f64,
    pub value2: f64,
}

fn main() {
    float_clone();

    struct_clone();
}

/// Clone a float.
///
/// The Clone trait is intrinsically implemented on the f64 type.
fn float_clone() {
    let new_float: f64 = 1000.0;
    println!("Original: {}", new_float);

    let clone_float = new_float.clone();
    println!("Clone: {}", clone_float);
}

/// Clone a structure.
///
/// Note that we must derive the Clone trait on the structure definition.  It doesn't implement it by default.
fn struct_clone() {
    let new_structure = TestStruct {
        value1: 1.0,
        value2: 2.0,
    };
    println!("Original: {:?}", new_structure);

    let clone_structure = new_structure.clone();
    println!("Clone: {:?}", clone_structure);
}
