-module(collatz_conjecture).

-export([test_version/0, steps/1]).

test_version() ->
  1.

steps(X) when X =< 0 -> {error, "Only strictly positive numbers are allowed"};
steps(X) -> do_steps(X, 0).

do_steps(1, Acc) -> Acc; 
do_steps(X, Acc) when X rem 2 == 0 -> do_steps(X div 2, Acc+1);
do_steps(X, Acc) -> do_steps(3 * X +1, Acc + 1).
