fib @ [n <- 3]
where
  dim n <- 0

  fib =
    if #.n <= 1 then
      #.n
    else
      (fib @ [n <- #.n - 1]) + (fib @ [n <- #.n - 2])
end