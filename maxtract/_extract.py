from extract import Extractor
from extract import patterns
from mapper import Node
from typing import List
from typing import Set
from sys import stderr
from error import NodeError


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
        for target in options.target:
            try:
                with open(target, "r") as file:
                    node_list += [Node(url.strip("\r\n")) for url in file.readlines()]
            except FileNotFoundError:
                print(f"Could not find file '{target}'.", file=stderr)
    else:
        node_list: Set[Node] = set()
        for url in options.target:
            try:
                node_list.add(Node(url))
            except NodeError:
                print(f"Error accessing url '{url}'.", file=stderr)

    extracted = Extractor(node_list, *extract_patterns).extract()
    for info in extracted:
        print(info)
