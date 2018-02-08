# gaia

A pure intensional programming language.

# Sample Program

```
fib @ [n <- 8]
where
  dim n <- 0

  fib =
    if #.n <= 1 then
      #.n
    else
      fib @ [n <- #.n - 1] + fib @ [n <- #.n - 2]
end
```
![Sample1](/output.png?raw=true "Sample Output")



