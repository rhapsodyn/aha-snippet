function mergeSort(arr) {
    if (arr.length <= 1) {
        return arr;
    }

    let middle = arr.length / 2;
    let left = arr.slice(0, middle);
    let right = arr.slice(middle, arr.length);

    let sortedLeft = mergeSort(left);
    let sortedRight = mergeSort(right);

    return merge(sortedLeft, sortedRight);
}

function merge(left, right) {
    let result = [];

    while (left.length !== 0 && right.length !== 0) {
        if (left[0] <= right[0]) {
            result.push(left.shift());            
        } else {
            result.push(right.shift());
        }
    }

    if (left.length !== 0) {
        result = result.concat(left);
    }

    if (right.length != 0) {
        result = result.concat(right);
    }

    return result;
}

console.log(mergeSort([232,12,3,4,123,8,3,13,5,2,1]));