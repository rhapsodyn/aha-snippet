//instead of return fibo(n-1) + fibo(n-2), saving stacks & more effective
function fibo(n) {
    let cache = [1, 1];

    for (let i = 2; i < n; i++ ) {
        cache[i] = cache[i - 1] + cache[i - 2];
    }

    return cache[n - 1];
}

console.log(fibo(10));