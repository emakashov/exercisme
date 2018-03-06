package scrabble

import (
	"strings"
)

func score(ch byte) int {
	switch ch {
	case 'A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T':
		return 1
	case 'D', 'G':
		return 2
	case 'B', 'C', 'M', 'P':
		return 3
	case 'F', 'H', 'V', 'W', 'Y':
		return 4
	case 'K':
		return 5
	case 'J', 'X':
		return 8
	case 'Q', 'Z':
		return 10
	default:
		return 0
	}
}

// Score give a word, compute the scrabble score for that word
func Score(input string) int {
	letters := []byte(strings.ToUpper(input))
	var sum int
	for i := 0; i < len(letters); i++ {
		sum += score(letters[i])
	}
	return sum
}
