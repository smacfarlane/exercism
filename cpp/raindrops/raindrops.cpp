#include "raindrops.h"

using namespace std;

namespace raindrops {
  string convert(int drops) {
    string result{""};

    if( drops % 3 == 0 ) { result += "Pling";}
    if( drops % 5 == 0 ) { result += "Plang";}
    if( drops % 7 == 0 ) { result += "Plong";}
    if(result == "") { result = to_string(drops);}

   return result;
  }
}  
