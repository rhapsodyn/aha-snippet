use std::fmt::Debug;

pub fn heap_sort<T: PartialOrd + Copy + Debug>(input: &mut [T]) {
    let n = input.len();
    let mut i = n / 2 - 1;

    // from bottom to top
    // make a bst
    loop {
        swap_down(input, n, i);
        if i > 0 {
            i -= 1;
        } else {
            break;
        }
    }
    dbg!(&input);

    i = n - 1;
    loop {
        // put every root (largest) to last
        swap(input, 0, i);
        swap_down(input, i, 0);
        dbg!(i, &input);
        if i > 0 {
            i -= 1;
        } else {
            break;
        }
    }
}

///
/// JUST ensure root of `tree` is the largest
///
fn swap_down<T: PartialOrd + Copy>(arr: &mut [T], n: usize, mid: usize) {
    let left = mid * 2 + 1;
    let right = mid * 2 + 2;
    let mut largest = mid;

    if left < n && arr[left] > arr[mid] {
        largest = left;
    }

    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != mid {
        swap(arr, largest, mid);
        swap_down(arr, n, largest);
    }
}

fn swap<T: PartialOrd + Copy>(arr: &mut [T], l: usize, r: usize) {
    let temp = arr[r];
    arr[r] = arr[l];
    arr[l] = temp;
}

#[test]
fn it_works() {
    let mut input = vec![1, 12, 9, 5, 6, 10];
    heap_sort(&mut input);
    assert_eq!(input, vec![1, 5, 6, 9, 10, 12]);
}
