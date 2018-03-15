// Fibonacci stream
//   * A stream of fibonacci numbers 'fib' varies according to a dimension 'n'.
//   * Values are cached along the identifier 'fib'.
fib @ [n <- 10]
where
  dim n <- 0

  fib =
    if #.n <= 1 then
      #.n
    else
      fib @ [n <- #.n - 1] + fib @ [n <- #.n - 2]
end