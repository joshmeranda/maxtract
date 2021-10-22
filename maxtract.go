package maxtract

import (
	"fmt"
	"github.com/gocolly/colly/v2"
	"net/url"
)

func Collect(url *url.URL, collector *colly.Collector) {
	collector.OnResponse(func(response *colly.Response) {
		requestUrl := response.Request.URL

		_ = requestUrl

		fmt.Printf("=== 001 %s ===\n", requestUrl)
	})

	collector.OnHTML("a[href]", func(element *colly.HTMLElement) {
		link := element.Attr("href")

		if hasVisited, _ := collector.HasVisited(link); ! hasVisited {
			fmt.Printf("=== 000 %s ===\n", link)
			_ = collector.Visit(link)
		}
	})

	_ = collector.Visit(url.String())

	collector.Wait()

	fmt.Println(collector.String())
}