package main

import (
	"encoding/json"
	"fmt"
	"github.com/akamensky/argparse"
	"github.com/gocolly/colly/v2"
	"github.com/jmeranda/maxtract"
	"net/url"
	"os"
	"regexp"
)

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

	var dataOnly, full, asJson, prettyJson *bool

	dataOnly = parser.Flag("o", "data-only", &argparse.Options{
		Validate: func(args []string) error {
			if *full || *asJson || *prettyJson {
				return fmt.Errorf("[-o,--data-only] ony one of --data-only, --full, --json, and --pretty-json may be specified")
			}

			return nil
		},
		Help: "only print the extracted data, without the source url (default))",
	})

	full = parser.Flag("f", "full", &argparse.Options{
		Validate: func(args []string) error {
			if *dataOnly || *asJson || *prettyJson {
				return fmt.Errorf("[-f,--full] ony one of --data-only, --full, --json, and --pretty-json may be specified")
			}

			return nil
		},
		Help: "print the source url as a heading before the found data",
	})

	asJson = parser.Flag("j", "json", &argparse.Options{
		Validate: func(args []string) error {
			if *dataOnly || *full || *prettyJson {
				return fmt.Errorf("[-j,--json] ony one of --data-only, --full, --json, and --pretty-json may be specified")
			}

			return nil
		},

		Help: "print the data as a single line of as_json",
	})

	prettyJson = parser.Flag("J", "pretty-json", &argparse.Options{
		Validate: func(args []string) error {
			if *dataOnly || *full || *asJson {
				return fmt.Errorf("[-J,--pretty-json] ony one of --data-only, --full, --json, and --pretty-json may be specified")
			}

			return nil
		},
		Help: "print the data as nicely formatted as_json",
	})

	var regex *regexp.Regexp
	parser.String("p", "pattern", &argparse.Options{
		Required: true,
		Validate: func(args []string) (err error) {
			switch pattern := args[0]; pattern {
			case "phone":
				regex = regexp.MustCompile("([0-9a-zA-Z]([-.[a-zA-Z0-9_]]*[0-9a-zA-Z])*@([0-9a-zA-Z][-[a-zA-Z0-9_]]*[0-9a-zA-Z]\\.)+[a-zA-Z]{2,9})")
			case "email":
				regex = regexp.MustCompile("\\([0-9]{3}\\)[ \t\n\r\f\v]?[0-9]{3}[-.][0-9]{4}|[0-9]{3}[-./[ \t\n\r\f\v]][0-9]{3}[-.[ \t\n\r\f\v]][0-9]{4}")
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

	rawDomains := parser.StringList("D", "domain", &argparse.Options{})

	allowAllDomains := parser.Flag("", "allow-all-domains", &argparse.Options{
		Help: "allow maxtract to craw outside of domain of the initial url, if '--domains' is specified this flag is ignored",
	})

	err := parser.Parse(os.Args)

	if ! (*dataOnly || *full || *asJson || *prettyJson) {
		*dataOnly = true
	}

	if err != nil {
		fmt.Println("Error:", err)
		return
	}

	domains := *rawDomains

	var options = []colly.CollectorOption{
		colly.MaxDepth(*maxDepth),
		colly.Async(true),
	}

	if err != nil {
		fmt.Println("Error:", err)
	}

	if len(domains) > 1 {
		*allowAllDomains = false
	}

	if ! *allowAllDomains {
		domains = append((domains)[:], rootUrl.Hostname())
		options = append(options, colly.AllowedDomains(domains...))
	}

	collector := colly.NewCollector(
		options...
	)

	nodes := maxtract.Collect(rootUrl, collector, regex)

	switch {
	case *dataOnly:
		// todo: create a set to avoid showing duplicates in the minimal data only output
		for _, node := range nodes {
			for _, datum := range node.Data {
				fmt.Println(datum)
			}
		}
	case *full:
		for _, node := range nodes {
			fmt.Println(node.Url.String())

			for _, datum := range node.Data {
				fmt.Println("├─", datum)
			}
		}
	case *asJson:
		out, err := json.Marshal(nodes)

		if err != nil {
			fmt.Println("Error:", err)
		} else {
			fmt.Println(string(out))
		}
	case *prettyJson:
		out, err := json.MarshalIndent(nodes, "  ", "  ")

		if err != nil {
			fmt.Println("Error:", err)
		} else {
			fmt.Println(string(out))
		}
	}
}
