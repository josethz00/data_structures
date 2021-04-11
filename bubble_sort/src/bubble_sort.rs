pub fn run() {
    let mut arr: [i32; 25] = [
        -223, -12, -1000, -90, -3, 
        40, 55, 11, 32, 67, 5, 74, 
        89, 38, 66, 27, 36, 79, 99, 
        2, 0, 1, 100, 282, 370
    ];
    
    let arr_length: usize = arr.len();
    let mut swap: i32;

    for _ in 0..arr_length {
        for j in 0..arr_length - 1 {
            if arr[j] > arr[j+1] {    
                swap = arr[j];
                arr[j] = arr[j+1];
                arr[j+1] = swap;
            }
        }
    }

    println!("Sorted array: {:?}", arr);
}