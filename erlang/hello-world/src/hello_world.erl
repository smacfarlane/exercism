-module(hello_world).


-export([test_version/0, hello/0]).

test_version() ->
  2.

hello() ->
  "Hello, World!".
