(=> test) @ [t <- 3]
where
  dim t <- 0

  test = intension @ [t <- 0]

  intension = {} #.t
end