"""Define the Node atomic class for the website map."""
from __future__ import annotations
from typing import Set
from typing import Union
from urllib import request
from bs4 import BeautifulSoup
from mapper import utils
from error import NodeError


class Node:
    """A node for each url, mapping the url to its html content and child links.

    Attributes:
        url (str): The url for the node.
        html (str); The html content received from the url.
        children (Set[str]): The set of child links

    Params:
        url (str): The url for the node.
    """
    def __init__(self, url: str, barren: bool = False):
        self.__barren = barren
        self.url = url[:len(url) - 1] if url[-1] == "/" else url # remove trailing slash
        self.html: str = self.__init_html()
        self._soup: BeautifulSoup = BeautifulSoup(self.html, "html5lib")
        self.children: Set[str] = set() if self.__barren else self.__init_children()

    def __iter__(self):
        """Provides a generator for iterating over the children of a node."""
        for child in self.children:
            yield child

    def __eq__(self, other: Union[Node, str]):
        """Two nodes can be considered equal if they share the same url.

        This is done to avoid costly equality checks with large html contents and with many child
        links.
        """
        is_equal: bool = False
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

    def __init_html(self) -> str:
        """Initialize the node's html content.

        Raises:
            NodeError: On error receiving html content.
        """
        html = str()

        try:
            with request.urlopen(self.url) as response:
                html += str(response.read())
        except Exception:
            raise NodeError(self.url)

        return html

    def __init_children(self) -> Set[str]:
        """Initialize the node's child set."""
        return {utils.normalize_link(self.url, a["href"])
                for a in self._soup.find_all("a", href=True)}

    def update(self):
        """Update the Node's html and children from the current values."""
        self.html = self.__init_html()
        self.children: Set[str] = set() if self.__barren else self.__init_children()
