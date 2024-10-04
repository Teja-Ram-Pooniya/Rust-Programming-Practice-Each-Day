fn main() {
     // Description About Vectors in Rust
     // Vectors in Rust
    // - Vectors (Vec) are similar to arrays, but they are dynamically sized, 
    //   meaning you can grow or shrink them.
    // - They are part of Rust's standard library and can be modified after 
    //    creation.
    
    // Declare a vector with 3 elements
    let mut vec: Vec<i32> = vec![10, 20, 30];
    
    // Add elements to the vector
    vec.push(40);
    vec.push(50);

    // Accessing vector elements
    println!("First element: {}", vec[0]);
    println!("Second element: {}", vec[1]);
    
    // Length of the vector
    println!("Length of the vector: {}", vec.len());

    // Loop through the vector
    for (i, value) in vec.iter().enumerate() {
        println!("Element at index {}: {}", i, value);
    }

    // Remove the last element from the vector
    vec.pop();
    println!("After pop, the vector is: {:?}", vec);
}
