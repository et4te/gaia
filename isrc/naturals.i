naturals @ [t <- 10]
where
    dim t <- 0

    naturals = fby.t 0 (naturals + 1)

    fby.t X Y = 
        if #.t <= 0 then
            X
        else
            Y @ [t <- #.t - 1]
end