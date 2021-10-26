package maxtract

import (
	"encoding/json"
	"fmt"
	"github.com/gocolly/colly/v2"
	"net/url"
	"regexp"
)

type MarshallingURL url.URL

func (link MarshallingURL) MarshalJSON() ([]byte, error) {
	return json.Marshal(link.String())
}

func (link MarshallingURL) String() string {
	lu := url.URL(link)
	return lu.String()
}

// CollectionNode is a wrapper around a webpage and the child links and data it contains.
type CollectionNode struct {
	Url MarshallingURL

	Children []MarshallingURL

	Data []string
}

func (node *CollectionNode) addChild(child url.URL) {
	node.Children = append(node.Children, MarshallingURL(child))
}

// Collect takes a preconfigured colly Collector and regular expression to extract Data
func Collect(root *url.URL, collector *colly.Collector, regex *regexp.Regexp) []*CollectionNode {
	nodes := make(map[url.URL]*CollectionNode, 0)

	writeLock := make(chan bool, 1)

	collector.OnResponse(func(response *colly.Response) {
		data := regex.FindAllString(string(response.Body), -1)

		node := CollectionNode{
			Url:      MarshallingURL(*response.Request.URL),
			Children: make([]MarshallingURL, 0),
			Data:     data,
		}

		writeLock <- true
		nodes[*response.Request.URL] = &node
		<- writeLock
	})

	collector.OnHTML("a[href]", func(element *colly.HTMLElement) {
		link := element.Attr("href")

		requestUrl := element.Request.URL

		linkUrl, err := url.Parse(link)

		if err != nil {
			fmt.Println("Error:", err)
			return
		}

		if ! linkUrl.IsAbs() {
			*linkUrl = *requestUrl
			linkUrl.Path = link
			linkUrl.Fragment = ""
		}

		if hasVisited, _ := collector.HasVisited(link); !hasVisited {
			writeLock <- true
			node := nodes[*requestUrl]

			node.addChild(*linkUrl)
			<-writeLock

			err = collector.Visit(linkUrl.String())
		}
	})

	_ = collector.Visit(root.String())

	collector.Wait()

	fmt.Println(collector)

	nodeList := make([]*CollectionNode, 0, len(nodes))

	for _, node := range nodes {
		nodeList = append(nodeList, node)
	}

	return nodeList
}
