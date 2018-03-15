// Running sum of fibonaccis
sum fib
where
    dim t <- 3
    dim n <- 3

    fby.t X Y = if #.t <= 0 then X else Y @ [t <- #.t - 1]

    sum S = fby.t S ((sum S) + 1)

    fib =
        if #.n <= 1 then
            #.n
        else
            fib @ [n <- #.n - 1] + fib @ [n <- #.n - 2]
end