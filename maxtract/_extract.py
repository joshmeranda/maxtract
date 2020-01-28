from extract import Extractor
from extract import patterns
from mapper import Node
from typing import List
from typing import Set


def run_extract(options):
    extract_patterns: List[str] = list()
    if options.email:
        extract_patterns.append(patterns.EMAIL)
    if options.phone:
        extract_patterns.append(patterns.PHONE_NUMBER)
    if options.regex:
        extract_patterns.append(options.regex)

    if options.file:
        node_list: List[Node] = list()
        for f in options.target:
            with open(f, "r") as file:
                node_list += [Node(url.strip("\r\n")) for url in file.readlines()]
    else:
        node_list: Set[Node] = {Node(url) for url in options.target}

    extracted = Extractor(node_list, *extract_patterns).extract()
    for info in extracted:
        print(info)
