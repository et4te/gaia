prev.d X = X @ [d <- #.d - 1]

next.d X = X @ [d <- #.d + 1]

fby.d X Y = 
    if #.d <= 0 then 
        X 
    else 
        Y @ [d <- #.d - 1]