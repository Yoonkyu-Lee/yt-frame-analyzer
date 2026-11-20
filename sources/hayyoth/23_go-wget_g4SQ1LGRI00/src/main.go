// Reconstructed from frames (24:56). The editor shows no line numbers or save
// messages. "package main" and the first imports were never on screen;
// fmt/io/log are inferred from use. Indentation as rendered (2 columns).
package main

import (
	"fmt"
	"io"
	"log"
	"net/http"
	"net/url"
	"os"
	"strings"

	"golang.org/x/net/html"
	"golang.org/x/net/html/atom"
)

func save(link string, content []byte) {
  u, err := url.Parse(link)
  if err != nil {
    log.Fatal(err)
  }
  link = u.Host + u.Path
  os.MkdirAll(link, 0777)

  if u.Path == "" {
    link += "/"
  }

  if strings.HasSuffix(link, "/") {
    link += "index.html"
  }

  fmt.Println("===> save ", link)

  // save file content
  os.WriteFile(link, content, 0777)
}

// done - simple
func crawl(link string) {
  client := &http.Client{}

  req, err := http.NewRequest("GET", link, nil)
  if err != nil {
    log.Fatal(err)
  }
  req.Header.Add("User-Agent", `Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:159.0) Gecko/20100101 Firefox/159.0`)
  resp, err := client.Do(req)
  if err != nil {
    log.Fatal(err)
  }

  defer resp.Body.Close()
  bodyByte, err := io.ReadAll(resp.Body)

  body := string(bodyByte)
  save(link, bodyByte)

  doc, err := html.Parse(strings.NewReader(body))
  if err != nil {
    log.Fatal(err)
  }
  for n := range doc.Descendants() {
    if n.Type == html.ElementNode && n.DataAtom == atom.A {
      for _, a := range n.Attr {
        if a.Key == "href" {
          subLink := a.Val

          if !strings.HasPrefix(subLink, "#") &&
            !strings.HasPrefix(subLink, "/") &&
            !strings.HasPrefix(subLink, "https") {
            crawl(link + "/" + subLink)
          }
          break
        }
      }
    }
  }
}

func main() {
  crawl("https://wingram.org")
}
