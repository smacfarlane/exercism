-module(accumulate).
-export([accumulate/2, test_version/0]).

test_version() ->
  1.

accumulate(_, []) -> [];
accumulate(Fn, [H|T]) ->
  [Fn(H)] ++ accumulate(Fn, T).
