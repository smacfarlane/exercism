package bob

import "strings"
import "regexp"

func Hey(remark string) string {
  trimmed_remark := strings.TrimSpace(remark); 
  if shouting(trimmed_remark) && has_letter(trimmed_remark) {
    return "Whoa, chill out!"
  } else if strings.HasSuffix(trimmed_remark, "?") {
    return "Sure."
  } else if len(trimmed_remark) == 0 {
    return "Fine. Be that way!"
  }

  return "Whatever."
}

func shouting(remark string) bool {
  return strings.ToUpper(remark) == remark
}

func has_letter(remark string) bool {
  matched, _ := regexp.Match("[A-Z]", []byte(remark))

  return matched
}

func question(remark string) bool {
  return strings.HasSuffix(remark, "?")
}
