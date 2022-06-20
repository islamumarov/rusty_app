fn main() {
    let mut array = [5, 3, 75, 2, 15, 76, 1, 4];
    quick_sort(&mut array);
    println!("{:?}", array);
}

// quick sort

fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot = arr[0];
    let mut i = 1;
    let mut j = 1;
    while j < arr.len() {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
        j += 1;
    }
    arr.swap(0, i - 1);
    quick_sort(&mut arr[0..i - 1]);
    quick_sort(&mut arr[i..]);
}
