#!/usr/bin/env python3
from maxtract._options import _parse_args
from argparse import Namespace
from typing import List, Set
from extract import patterns, Extractor
from mapper import Node
from error import NodeError
from sys import stderr, stdout
from mapper.utils import generate_map, make_local_copy


def main():
    options: Namespace = _parse_args()

    extract_patterns: List[str] = list()
    if options.email:
        extract_patterns.append(patterns.EMAIL)
    if options.phone:
        extract_patterns.append(patterns.PHONE_NUMBER)
    if options.regex:
        extract_patterns.append(options.regex)

    out = stdout if not options.file else options.file

    if options.depth:
        node_list = generate_map(options.url, not options.travel, options.depth, out)
    else:
        node_list = generate_map(options.url, not options.travel, file=out)

    if options.copy:
        make_local_copy(list(node_list), options.path)

    node_list: Set[Node] = set()
    for url in options.target:
        try:
            node_list.add(Node(url))
        except NodeError:
            print(f"Error accessing url '{url}'.", file=stderr)

    extracted = Extractor(node_list, *extract_patterns).extract()
    for info in extracted:
        print(info)


if __name__ == "__main__":
    main()
