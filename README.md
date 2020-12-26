# maxtract
Command line tool to allow for finding all urls from a website and extract data.

## Behavior
This tool is similar to a recursive grep using `grep --extended-regexp --recursive` on a directory tree. Maxtract
searching a website for all explicit links (links provided via `<a href="...">...</a>`) and pulls information from each
page.

When limiting the search depth via `--max-depth` be aware that a depth of 0 will only search the given url and no others.
It may be minimally faster to simply use `curl <url> | grep -E <pattern>` to extract information.

## Output
This tool can provide output in one of 3 ways as specified in the usage:

```
Output:
-o, --data-only      only print the extracted data, without the source url
-f, --full           print the url as a heading before the found data (default)
-j, --json           print the data as json
    --pretty-json    print the data as pretty json
```

#### Full
Prints the parent url and the extracted data in a tree-like format
```
$> maxtract https://sample.com/index.html --email
https://sample.com/child_00.html
├─ test.email@test.org
https://sample.com/child_01.html
├─ test.email@test.edu
https://sample.com/child_10.html
├─ test.email@test.co
https://sample.com/index.html
├─ i_am_an_email@some_school.edu
├─ test.email@test.com
```

#### Data Only
Only prints the extracted data on separate lines

```
$> maxtract https://sample.com/index.html --email --data-only
test.email@test.org
test.email@test.edu
test.email@test.co
i_am_an_email@some_school.edu
test.email@test.com
```

#### Json
Prints the entire map as json maintaining a list of the child urls from each parent. The output can be nicely formatted
(`--json-pretty`) or output on one line (`--json`).

```
$> maxtract https://sample.com/index.html --email --json-pretty
{
  "https://sample.com/child_00.html": {
    "url": "https://sample.com/child_00.html",
    "data": [
      "test.email@test.org"
    ],
    "children": [
      "https://sample.com/child_10.html"
    ]
  },
  "https://sample.com/child_01.html": {
    "url": "https://sample.com/child_01.html",
    "data": [
      "test.email@test.edu"
    ],
    "children": []
  },
  "https://sample.com/child_10.html": {
    "url": "https://sample.com/child_10.html",
    "data": [
      "test.email@test.co"
    ],
    "children": [
      "https://sample.com/index.html"
    ]
  },
  "https://sample.com/index.html": {
    "url": "https://sample.com/index.html",
    "data": [
      "i_am_an_email@some_school.edu",
      "test.email@test.com"
    ],
    "children": [
      "https://sample.com/child_00.html",
      "https://sample.com/child_01.html"
    ]
  }
}
```