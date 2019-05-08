function mergeSort(arr) {
  const final = (function recur(arrOfArr) {
    if (arrOfArr.length === 1) {
      return arrOfArr;
    }

    const newArr = [];
    for (let i = 0; i < arrOfArr.length; i += 2) {
      if (i === arrOfArr.length - 1) {
        newArr.push(arrOfArr[i]);
      } else {
        newArr.push(_merge(arrOfArr[i], arrOfArr[i + 1]));
      }
    }
    return recur(newArr);
  })(arr.map(a => [a]));
  return final[0];
}

function _merge(arr1, arr2) {
  const arr = [];
  let c1 = 0;
  let c2 = 0;
  while (c1 != arr1.length || c2 != arr2.length) {
    if (c1 === arr1.length) {
      arr.push(...arr2.slice(c2, arr2.length));
      break;
    }
    if (c2 === arr2.length) {
      arr.push(...arr1.slice(c1, arr1.length));
      break;
    }
    if (arr1[c1] < arr2[c2]) {
      arr.push(arr1[c1]);
      c1++;
    } else {
      arr.push(arr2[c2]);
      c2++;
    }
  }
  return arr;
}

console.log(mergeSort([1, 5, 345, 123, 53, 6, 2, 776, 1235, 10]));
