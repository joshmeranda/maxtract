#!/usr/bin/env python3
import argparse
import asyncio
import typing
import urllib.parse as parse

import maxtract
import maxtract.extract as extract
import maxtract.traverse as traverse


def options():
    """Parses command line arguments."""
    maxtract = argparse.ArgumentParser(prog="maxtract", add_help=True,
                                       epilog="If no extraction pattern is specified, the set of traversed links is returned",
                                       formatter_class=argparse.MetavarTypeHelpFormatter,
                                       description="traverse and extract information from a website")
    maxtract.add_argument("-v", "--verbose", action="store_true", help="show the url being traversed")

    maxtract.add_argument("root_url", action="store", type=str, metavar="url",
                          help="the url to start the site traversal one")

    maxtract.add_argument("-d", "--depth", action="store", type=int, default=-1,
                          help="the greatest depth of links to follow")

    maxtract.add_argument("-e", "--extract", action="store", nargs="+", type=str, metavar="patterns",
                          help="the regex pattern(s) to extract or a supported alias ('phone' and 'email')")

    maxtract.add_argument("-c", "--copy", action="store", type=str,
                          help="make a local copy of the retrieved nodes (links will not be resolved to the local version)")

    return maxtract.parse_args()


def main():
    opts = options()

    nodes: typing.Set[traverse.Node] = set()

    if not opts.verbose:
        maxtract.verbose_print = lambda *args: None

    asyncio.run(traverse.traverse_site(opts.root_url, opts.depth, nodes, parse.urlparse(opts.root_url).netloc))

    # clear and reset output line
    maxtract.verbose_print("")

    if opts.extract:
        patterns = list()
        for pattern in opts.extract:
            if pattern == "phone":
                patterns.append(extract.Patterns.PHONE_NUMBER)
            elif pattern == "email":
                patterns.append(extract.Patterns.EMAIL)
            else:
                patterns.append(patterns)

        ext = extract.Extractor(nodes, *patterns)

        # clear and reset output line
        maxtract.verbose_print("")

        [print(data) for data in ext.extract()]
    else:
        [print(node.url) for node in nodes]

    if opts.copy is not None:
        traverse.make_local_copy(nodes, opts.copy)


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        pass
