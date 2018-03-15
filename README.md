# gaia

This project is codenamed 'gaia' until a more suitable name is found. 

This repository is an implementation of a programming language which is based on possible world semantics and in particular allows for the definition of intensions which can be used as the basis for the implementation of various incarnations of modal logic. The implementation uses the latest research in the intensional semantics world where TransLucid is the latest body of work which formally specifies important semantics which were missing from prior works (such as higher order functions).

The goal is to arrive at a usable language and set of semantics for running programs in a distributed setting and in accordance with a dynamic cryptographic protocol, a 'language of the network' in more abstract parlance. Given that the language relies solely on distributed environments (unlike languages which rely on closures), every intension is globally distributable. 

The status of the project is that a programming language has been developed which still needs some work in order to be usable in a distributed setting and a concrete mapping to a distributed network such as extended Kademlia is in the works which promotes the current caching mechanism to a global environment. 

# Functional Fibonacci

In order to introduce the programming language, let us take a look at what a program in a traditional programming language might look like. As a simple example we define a function which computes the fibonacci of 3.

```
fib!3
where
  fib!n = 
    if n <= 1 then 
      n 
    else 
      (fib!(n - 1)) + (fib!(n - 2)) 
end
```

This program can be read as follows, compute the fib!3 where the fib!n is a function whose argument `n` _is applied eagerly_ and whose body is the fibonacci conditional.

Lo and behold, we obtain the number 2. Nothing particularly scary going on and in fact there is nothing different here than in any regular programming language which implements recursive function definitions such as ML.

# Intensional Fibonacci

The same program can be rewritten intensionally as follows.

```
fib @ [n <- 3]
where
  dim n <- 0

  fib = 
    if #.n <= 1 then 
      #.n 
    else 
      fib @ [n <- #.n - 1] + fib @ [n <- #.n - 2]
end
```

Where we introduce the context switching operator `@`, tuple definition expressions enclosed in `[` and `]` and dimension declarations `dim x <- n`.  

The program can be read as, in order to obtain the fibonacci _at dimension n of 3_ we create a local dimension n whose default value is 0 and we specify the fibonacci sequence as an _intension_ which contains the fibonacci conditional varying along the dimension n.

Initially this may look like it would produce the same result as the program above and indeed the resulting value of running this program is still 2 given that the initial expression asks for the fibonacci at that point in space however the computation of the result is different in that here fib represents an infinite stream of computed values which are _cached globally and immutably_.

This has the result of taking up more space but produces a deterministic (and if desired unique) _extension_ upon evaluation. If a distributed node were to share the same cache as is common place in current blockchain based nodes, the `fib @ [n <- 3]` would have been retrieved in constant time from the first reachable node which computed it.  

Thus in this case, the result of the program is not just 2, it is also the extension built which led to the final outcome.

```
fib [n <- 0] = 0
fib [n <- 1] = 1
fib [n <- 2] = 1
fib [n <- 3] = 2
```

# Infinite Naturals

In order to compute an infinite stream of naturals, we can use the following more concise equation.

```
dim t <- 0

naturals = fby.t 0 (naturals + 1)
```

Where a full program requesting a natural and its extension at time 10 and defining fby looks as follows.

```
naturals @ [t <- 10]
where
  dim t <- 0

  naturals = fby.t 0 (naturals + 1)

  fby.d X Y =
    if #.d <= 0 then
      X
    else
      Y @ [d <- #.d - 1]
end
```

From the above example we have defined a new type of function - one which varies across any given dimension 'd' where fby.t means 'followed by in the t dimension'. This allows us to specify the complete set of natural numbers across the t dimension concisely.

In future iterations of the language it won't be necessary to define the standard operators since they will be added to a prelude but for now it is more instructive to see their definitions.

Other convenient operators which help specify concise equations which vary multidimensionally are as follows:
```
// first.d (X at d of 0)
first.d X = X @ [d <- 0]

// prev.d (X at the previous d)
prev.d X = X @ [d <- #.d - 1]

// next.d (X at the next value of d)
next.d X = X @ [d <- #.d + 1]

// wvr.d (whenever Y, retain elements of X)
wvr.d X Y = 
  if first.d Y then
    fby.d X (wvr.d (next.d X) (next.d Y))
  else
    wvr.d (next.d X) (next.d Y)

// asa.d (as soon as P, return X)
asa.d X Y = first.d (wvr.d X Y)

// upon.d (advance upon X when Y is true)
upon.d X Y = X @ [d <- Z]
where
  Z = fby.d 0 (if Y then Z + 1 else Z)
end
```



