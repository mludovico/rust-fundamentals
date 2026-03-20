fn main() {
    let mut vec = vec![1, 2, 3];
    println!("Original vector: {:?}", vec);

    // Adding elements to the end of the vector
    vec.push(4);
    println!("After push: {:?}", vec);

    // Extending the vector with another vector
    let more_nubers = vec![5, 6];
    vec.extend(more_nubers);
    println!("After extend: {:?}", vec);

    // Appending elements from an array
    let mut other_numbers = vec![7, 8, 9];
    vec.append(&mut other_numbers);
    println!("After extend_from_slice: {:?}", vec);

    // Inserting an element at a specific index
    vec.insert(0, 0);
    println!("After insert: {:?}", vec);
}
