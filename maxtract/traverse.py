"""Provide classes and utils for finding links from a website."""
from __future__ import annotations

import os
import typing
import urllib.parse as parse
from typing import Set, Union
import pathlib

import requests
from bs4 import BeautifulSoup

import maxtract


class NodeError(Exception):
    """Error instantiating a new Node instance."""

    def __init__(self, url: str, cause: str):
        super(NodeError, self).__init__(f"Could not instantiate node with url {url}: {cause}")


class Node:
    """A node for each url, mapping the url to its html content and child links.

    :param url: THe url for the node.
    """

    def __init__(self, url: str):
        self.url = url.rstrip("/")  # remove trailing slash

        # retrieve the html at the given url.
        try:
            request = requests.get(self.url, timeout=1)

            if not request.ok:
                raise NodeError(self.url, "Not Found")

            self.html = request.text
        except requests.Timeout:
            raise NodeError(self.url, "Timeout")

        self._soup: BeautifulSoup = BeautifulSoup(self.html, "html5lib")

        # scrape all children links from the url html
        self.children: Set[str] = {normalize_link(self.url, a["href"])
                                   for a in self._soup.find_all("a", href=True)}

    def __eq__(self, other: Union[Node, str]):
        """Two nodes can be considered equal if they share the same url.

        This is done to avoid costly equality checks with large html files with many children.
        """
        if isinstance(other, str):
            is_equal = other == self.url
        elif isinstance(other, Node):
            is_equal = other.url == self.url
        else:
            raise TypeError

        return is_equal

    def __str__(self) -> str:
        return self.url

    def __repr__(self) -> str:
        return f"<Node url={self.url}, html={self.html}, children={self.children}>"

    def __lt__(self, other: Node) -> bool:
        """Allows for sorting collections of Nodes."""
        return self.url < other.url

    def __hash__(self) -> int:
        """Allows for storing Nodes in a set in reference to their url."""
        return hash(self.url)


async def traverse_site(root_node: typing.Union[str, Node], depth: int = -1, visited: typing.Set[Node] = set(),
                        domain: typing.Optional[str] = None):
    """Traverse a website by finding all links on the website, and printing the urls

    :param root_node: The Node or url to start the traversal from.
    :param depth: The depth of the site tree that should be iterated over.
    :param visited: A set with all the Nodes which have already been traversed.
    :param domain: If specified urls outside of the domain will not be traversed.
    :return: The set of traversed nodes.
    """
    if isinstance(root_node, str):
        try:
            root_node = Node(root_node)
        except NodeError as err:
            maxtract.verbose_print(f"{str(err)}\n")
            return

    # print the node being traversed
    maxtract.verbose_print(f"traversing {root_node.url}")

    visited.add(root_node)

    if depth == 0:
        return {root_node}

    for child in root_node.children:
        if domain is not None:
            child_domain = parse.urlparse(child).netloc

            if child_domain != domain:
                # print(child, child_domain, domain, "\n")
                continue

        child = child.rstrip("/")

        if child not in visited:
            await traverse_site(child, depth - 1, visited, domain)

    return


def normalize_link(parent: str, child: str) -> typing.Optional[str]:
    """Normalize a child link against the url in which the link was found.

    Args:
        parent (str): The parent path for the sub-link.
        child (str): The path to normalize.

    Returns:
        Optional[str]: The normalized path or None if path could not be normalized.
    """
    cattr: parse.ParseResult = parse.urlparse(child)  # Child attributes

    # if the link already has a scheme, it does not need further normalizations
    if cattr.scheme:
        return child

    pattr: parse.ParseResult = parse.urlparse(parent)  # Parent attributes
    url: str = pattr.scheme + "://"

    if pattr.netloc != "":
        url += pattr.netloc

    if os.path.isabs(cattr.path):
        url += os.path.normpath(cattr.path)
    else:
        url += os.path.normpath(f"{os.path.dirname(pattr.path)}/{cattr.path}")

    if cattr.params != "":
        url += ";" + cattr.params

    if cattr.query != "":
        url += "?" + cattr.query

    if cattr.fragment != "":
        url += "#" + cattr.fragment

    return url


def make_local_copy(source: typing.Set[Node], path: str):
    """Create a local duplicate of the given map.

    Please note that most websites have a large amount of pages which hold a lot of memory, so
    please be sure you have enough storage space for the duplicated data.

    Params:
        source (Set[Nodes]: The set of nodes to replicate locally.
        path (str): The root path for the duplicated data.
    """
    for node in source:
        attr: parse.ParseResult = parse.urlparse(node.url)

        if not attr.path:
            continue

        html: str = node.html
        full_path: str = str(pathlib.Path(path).resolve()) + attr.path

        if os.path.isdir(full_path):
            continue

        if not os.path.exists(os.path.dirname(full_path)):
            print(full_path)
            os.makedirs(os.path.dirname(full_path), 0o755, exist_ok=True)

        with open(full_path, 'w+') as file:
            file.write(html)
