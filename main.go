package main

import (
	"fmt"
	"github.com/akamensky/argparse"
	"github.com/gocolly/colly"
	"net/url"
	"os"
	"regexp"
)

func collect(url *url.URL, maxDepth int) {
	fmt.Println("=== 000", url.Hostname(), "===")

	filterRegex := regexp.MustCompile(fmt.Sprintf(".*%s.*", url.Hostname()))

	collector := colly.NewCollector(
		colly.URLFilters(filterRegex),
		colly.MaxDepth(maxDepth),
		colly.Async(true))

	collector.OnHTML("a[href]", func(element *colly.HTMLElement) {
		link := element.Attr("href")

		// todo: do some processing

		err := collector.Visit(link)

		if err != nil {
			// do nothing...
		}
	})

	collector.OnRequest(func(request *colly.Request) {
		fmt.Printf("=== 001 '%s' ===\n", request.URL)
	})

	err := collector.Visit(url.String())

	if err != nil {
		fmt.Println("Error:", err)
	}

	collector.Wait()
}

func main() {
	parser := argparse.NewParser("maxtract", "A command line tool for extracting information from websites")

	parser.ExitOnHelp(true)

	maxDepth := parser.Int("d", "max-depth", &argparse.Options{
		Validate: func(args []string) error {
			if args[0][0] == '-' {
				return fmt.Errorf("[-d|--max-depth] value must be >= 0")
			}

			return nil
		},
		Help:    "the maximum depth of links to walk down (must be >= 0) if 0 (the default) there will be not depth limit",
		Default: 0,
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

	var rootUrl *url.URL
	parser.String("u", "url", &argparse.Options{
		Required: true,
		Validate: func(args []string) (err error) {
			parsedUrl, err := url.Parse(args[0])

			rootUrl = parsedUrl

			return err
		},
		Help: "the url at which to start the search",
	})

	err := parser.Parse(os.Args)

	if err != nil {
		fmt.Println("Error:", err)
		return
	}

	if err != nil {
		fmt.Println("Error:", err)
	} else if true {
	} else {
		fmt.Printf("=== %t %t %t %t ===\n", *dataOnly, *full, *json, *prettyJson)
		fmt.Printf("=== %d ===\n", *maxDepth)
		fmt.Printf("=== %s ===\n", rootUrl.String())
		fmt.Printf("=== %s ===\n", regex.String())
	}

	collect(rootUrl, *maxDepth)
}
