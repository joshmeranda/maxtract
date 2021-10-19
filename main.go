package main

import (
	"fmt"
	"github.com/akamensky/argparse"
	"os"
	"regexp"
)

func main() {
	parser := argparse.NewParser("maxtract", "A command line tool for extracting information from websites")

	maxDepth := parser.Int("d", "max-depth", &argparse.Options{
		Validate: func(args []string) error {
			if args[0][0] == '-' {
				return fmt.Errorf("[-d|--max-depth] value must be >= 0")
			}

			return nil
		},
		Help:    "the maximum depth of links to walk down (must be >= 0). If 0 only the content at the given url will be search",
		Default: -1,
	})

	dataOnly := parser.Flag("o", "data-only", &argparse.Options{
		Help: "only print the extracted data, without the source url",
	})
	full := parser.Flag("f", "full", &argparse.Options{
		Help: "print the source url as a heading before the found data (default)",
	})
	json := parser.Flag("j", "json", &argparse.Options{
		Help: "print the data as a single line of json",
	})
	prettyJson := parser.Flag("J", "pretty-json", &argparse.Options{
		Help: "print the data as nicely formatted json",
	})

	var regex *regexp.Regexp

	_ = parser.String("p", "pattern", &argparse.Options{
		Required: true,
		Validate: func(args []string) (err error) {
			switch pattern := args[0]; pattern {
			case "phone":
				regex, err = regexp.Compile("([0-9a-zA-Z]([-.\\w]*[0-9a-zA-Z])*@([0-9a-zA-Z][-\\w]*[0-9a-zA-Z]\\.)+[a-zA-Z]{2,9})")
			case "email":
				regex, err = regexp.Compile("\\(\\d{3}\\)\\s?\\d{3}[-.]\\d{4}|\\d{3}[-./\\s]\\d{3}[-.\\s]\\d{4}")
			default:
				regex, err = regexp.Compile(pattern)
			}

			return
		},
		Help: "the pattern to search for, can be one of phone, email, or a custom regex pattern",
	})

	url := parser.String("u", "url", &argparse.Options{
		Required: true,
		Help:     "the url at which to start the search",
	})

	err := parser.Parse(os.Args)

	if err != nil {
		fmt.Println("Error:", err)
	} else {
		fmt.Printf("=== %t %t %t %t ===\n", *dataOnly, *full, *json, *prettyJson)
		fmt.Printf("=== %d ===\n", *maxDepth)
		fmt.Printf("=== %s ===\n", *url)
		fmt.Printf("=== %s ===\n", regex.String())
	}
}
