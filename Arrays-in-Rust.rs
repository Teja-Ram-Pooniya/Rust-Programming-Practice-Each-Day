fn main() {
    // Declare an array of 5 integers
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    
    // Accessing array elements
    println!("First element: {}", arr[0]);
    println!("Second element: {}", arr[1]);
    
    // Length of the array
    println!("Length of the array: {}", arr.len());

    // Loop through the array
    for i in 0..arr.len() {
        println!("Element at index {}: {}", i, arr[i]);
    }

    // Using a slice to get part of the array
    let slice = &arr[1..3]; // Slice containing elements 2 and 3
    println!("Slice of the array: {:?}", slice);
}
