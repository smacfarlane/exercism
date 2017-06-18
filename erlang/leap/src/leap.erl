-module(leap).

-export([test_version/0, leap_year/1]).


test_version() ->
  1.

% on every year that is evenly divisible by 4
%   except every year that is evenly divisible by 100
%     unless the year is also evenly divisible by 400

leap_year(Year) when Year rem 4 == 0, Year rem 100 /= 0; Year rem 400 == 0 -> true;
leap_year(_) -> false.

