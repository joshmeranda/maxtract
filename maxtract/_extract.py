from extract import Extractor
from extract import patterns
from typing import List
from mapper import Node


def run_extract(options):
    extract_patterns: List[str] = list()
    if options.email:
        extract_patterns.append(patterns.EMAIL)
    if options.phone:
        extract_patterns.append(patterns.PHONE_NUMBER)
    if options.regex:
        extract_patterns.append(options.regex)

    node_list: List[Node] = [Node(url) for url in options.url]

    extracted = Extractor(node_list, *extract_patterns).extract()
    for info in extracted:
        print(info)
