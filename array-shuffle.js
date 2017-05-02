//https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle
//shuffle array in O(n) and convincible
function fisherYates(arr) {
    let result = arr.slice(0);

    for (var i = 0; i < result.length - 1; i++) {
        let num = result[i];
        let randomSlot = randomInt(i, result.length);

        let temp = result[randomSlot];
        result[randomSlot] = num;
        result[i] = temp;
    }
    
    return result; 
}

function randomInt(min, max) {
    min = Math.ceil(min);
    max = Math.floor(max);

    return Math.floor(Math.random() * (max - min)) + min;
}

console.log(fisherYates([1,2,3,4,5,6,7,8,9,10]));