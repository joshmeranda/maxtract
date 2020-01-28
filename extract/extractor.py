"""Define classes for extraction abstraction."""
from bs4 import BeautifulSoup
import re
from typing import Set
from mapper.node import Node


class Extractor:
    """Extracts data from the urls provided.

    Attributes:
        targets (list): The list of urls from which to extract data.
        pattern (str): The regular expression to use when extracting data.

    Params:
        targets (list): The list of urls from which to extract data.
        pattern (str): The regular expression to use when extracting data.
    """
    IGNORE = ["style", "script"]

    def __init__(self, targets: Set[Node], *patterns: str):
        self.targets = targets
        self.pattern = re.compile("|".join(patterns))

    def extract(self) -> Set[str]:
        """Pull all text matching the instances regex pattern."""
        data: Set[str] = set()

        for node in self.targets:
            found = set(re.findall(self.pattern, self._clean_html(node.html)))
            data = data.union(found)

        return data

    @staticmethod
    def _clean_html(html: str) -> str:
        """Removes unwanted tag types from the html.

        Params:
            html (str): The html to be cleaned

        Returns:
            (str): The cleaned html content.
        """
        soup = BeautifulSoup(html, "html5lib")

        for item in soup.find_all(Extractor.IGNORE):
            item.decompose()

        return soup.get_text("\n")

