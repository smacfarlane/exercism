// This is a "stub" file.  It's a little start on your solution.
// It's not a complete solution though; you have to write some code.

// Package acronym should have a package comment that summarizes what it's about.
// https://golang.org/doc/effective_go.html#commentary
package acronym

import "strings"
import "unicode"

func Abbreviate(s string) string {

  f := func(c rune) bool {
    return  unicode.IsSpace(c) || unicode.IsPunct(c)
  }

  var result = ""
  
  for _, substring := range strings.FieldsFunc(s, f) {
    result += string([]rune(substring)[0])
  }

  return strings.ToUpper(result)
}
