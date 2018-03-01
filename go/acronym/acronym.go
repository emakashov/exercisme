package acronym

import (
	"regexp"
	"strings"
)

// Abbreviate should have a comment documenting it.
func Abbreviate(s string) string {
	re := regexp.MustCompile(`[A-Za-z]+`)
	var res []string
	for _, word := range re.FindAllString(s, -1) {
		res = append(res, word[:1])
	}
	return strings.ToUpper(strings.Join(res, ""))
}
