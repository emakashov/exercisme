package raindrops

import (
	"strconv"
	"strings"
)

//
func Convert(input int) string {
	var buffer []string
	if input%3 == 0 {
		buffer = append(buffer, "Pling")
	}
	if input%5 == 0 {
		buffer = append(buffer, "Plang")
	}
	if input%7 == 0 {
		buffer = append(buffer, "Plong")
	}
	if len(buffer) > 0 {
		return strings.Join(buffer, "")
	}
	return strconv.Itoa(input)
}
