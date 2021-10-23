package maxtract

import (
	"fmt"
	"github.com/gocolly/colly/v2"
	"net/url"
	"regexp"
)

type collectionNode struct {
	Url url.URL

	Children []url.URL

	Data []string
}

func (node *collectionNode) addChild(child url.URL) {
	node.Children = append(node.Children, child)
}

// Collect takes a preconfigured colly Collector and regular expression to extract Data
// todo: might be worthwhile to take a storage destination
func Collect(root *url.URL, collector *colly.Collector, regex *regexp.Regexp) []*collectionNode {
	nodes := make(map[url.URL]*collectionNode, 0)

	collector.OnResponse(func(response *colly.Response) {
		data := regex.FindAllString(string(response.Body), -1)

		node := collectionNode{
			Url:      *response.Request.URL,
			Children: make([]url.URL, 0),
			Data:     data,
		}

		nodes[*response.Request.URL] = &node
	})

	collector.OnHTML("a[href]", func(element *colly.HTMLElement) {
		link := element.Attr("href")
		requestUrl := element.Request.URL

		if hasVisited, _ := collector.HasVisited(link); ! hasVisited {
			linkUrl, err := url.Parse(link)

			if err != nil {
				fmt.Println("Error:", err)
			}

			node := nodes[*requestUrl]
			node.addChild(*linkUrl)

			_ = collector.Visit(link)
		}
	})

	_ = collector.Visit(root.String())

	collector.Wait()

	nodeList := make([]*collectionNode, 0, len(nodes))

	for _, node := range nodes {
		nodeList = append(nodeList, node)
	}

	return nodeList
}