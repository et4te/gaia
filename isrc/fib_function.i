// Fibonacci by value
//   * A strict function 'fib' is defined which computes its arguments eagerly.
//   * Nothing is added to the cache and the computation is done in-place.
fib!10
where
    fib!n = 
        if n <= 1 then
            n
        else
            (fib!(n - 1)) + (fib!(n - 2))
end