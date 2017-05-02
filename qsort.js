function qsort(arr) {
    if (arr.length <= 1) {
        return arr;
    }

    let pivot = arr[0];
    let bigger = [];
    let smaller = [];
    for (var i = 1; i < arr.length; i++) {
        if (arr[i] <= pivot) {
            smaller.push(arr[i]);
        } else {
            bigger.push(arr[i]);
        }
    }

    return [].concat(qsort(smaller), pivot, qsort(bigger));
}

console.log(qsort([232,12,3,4,123,8,3,13,5,2,1]));