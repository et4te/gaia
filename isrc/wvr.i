wvr.t naturals (fby.t true false)
where
  dim t <- 0
  dim n <- 0

  naturals = fby.n 0 (naturals + 1)

  first.d X = X @ [d <- 0]

  next.d X = X @ [d <- #.d + 1]

  fby.d X Y =
    if #.d <= 0 then
      X
    else
      Y @ [d <- #.d - 1]

  wvr.d X Y =
    if first.d Y then
      fby.d X (wvr.d (next.d X) (next.d Y))
    else
      wvr.d (next.d X) (next.d Y)
end