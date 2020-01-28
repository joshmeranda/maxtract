"""Provides utility functions for creating website maps."""
from __future__ import annotations
import os.path
from math import inf
from sys import stderr
from typing import List
from typing import Set
from typing import Optional
from typing import TYPE_CHECKING
from urllib import parse
from error import NodeError

# resolve cyclical imports and allow for Node type hints
if TYPE_CHECKING:
    from mapper import Node
    from urllib.parse import ParseResult


def normalize_link(parent: str, child: str) -> Optional[str]:
    """Normalize a child link against the url in which the link was found.

    Args:
        parent (str): The parent path for the sub-link.
        child (str): The path to normalize.

    Returns:
        Optional[str]: The normalized path or None if path could not be normalized.
    """
    cattr: ParseResult = parse.urlparse(child)  # Child attributes

    # if the link already has a scheme, it does not need further normalizations
    if cattr.scheme:
        return child

    pattr: ParseResult = parse.urlparse(parent)  # Parent attributes
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


def generate_map(root_url: str, local: bool = True, depth: int = inf) -> Set[Node]:
    """Generate and return the web map starting from the given url.

    If local is true, links which would lead to external domains will still be listed as a child of
    a link; however, no additional entry will be added for said link as is done for local links.
        (ex):
            {"https://example.org": ["https://www.google.com", "https://example.org/robots.txt"]
             "https://example.org/robots.txt": []}

    Args:
        root_url (str): The root url from which to begin mapping the site.
        local (bool): Whether to follow links that  would lead away from the current domain.
        depth (int): The greatest depth the link tree should grow to.

    Returns:
        LinkMap: The generate map.
    """
    # pylint: disable=import-outside-toplevel
    # pylint: disable=cyclic-import
    from mapper import Node

    lmap = set()
    queue: List[Optional[str]] = [root_url, None]
    domain: Optional[str] = parse.urlparse(root_url).netloc if local else None

    while queue and depth > 0 or (not lmap and depth == 0):
        parent = queue.pop(0)
        if parent is None:
            depth -= 1
            continue

        try:
            node = Node(parent, barren=(depth == 0))
        except NodeError as error:
            print(f"Could not initialize node with url '{error.url}'",
                  file=stderr)
            continue

        lmap.add(node)

        for child in node:
            if child not in lmap:
                if local and parse.urlparse(child).netloc != domain:
                    continue
                queue.append(child)

    return lmap


def make_local_copy(source: List[Node], path: str):
    """Create a local duplicate of the given map.

    Please note that most websites have a large amount of pages which hold a lot of memory, so\
    please be sure you have enough storage space for the duplicated data.

    Params:
        source (Set[Nodes]: The set of nodes to replicate locally.
        path (str): The root path for the duplicated data.
    """
    for node in source:
        attr: ParseResult = parse.urlparse(node.url)
        html: str = node.html
        full_path: str = path + attr.path

        if not os.path.exists(os.path.dirname(full_path)):
            os.makedirs(os.path.dirname(full_path), 0o755, exist_ok=True)

        with open(full_path, 'w+') as file:
            file.write(html)
