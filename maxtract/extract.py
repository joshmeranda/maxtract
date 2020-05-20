"""Define classes for extraction abstraction."""
import re
from typing import Set

from bs4 import BeautifulSoup

import maxtract
from maxtract.traverse import Node


class Patterns:
    """Provides sample regex patterns to use for extracting data.

    The following patterns are provided:
        EMAIL: Matches emails.
        PHONE_NUMBER: Matches phone numbers.
    """
    EMAIL: str = "[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?" \
                 "(?:\\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*"

    PHONE_NUMBER: str = "(?:\\+?\\d{1,3}[\\s-]?)?" \
                        "(?:(?:\\(?\\d{3}\\)?)|(?:\\d{3}))" \
                        "(?:[\\s.-]?)" \
                        "(?:\\d{3})" \
                        "(?:[\\s.-]?)" \
                        "(?:\\d{4})"


class Extractor:
    """Extracts data from the urls provided.

    :param targets: The set of urls from which to extract data.
    :param patterns: The vararg patterns to use for data extraction.
    """
    _IGNORE = ["style", "script"]

    def __init__(self, targets: Set[Node], *patterns):
        self.targets = targets
        self.pattern = re.compile("|".join(patterns))

    def extract(self) -> Set[str]:
        """Pull all text matching the instances regex pattern.

        :return; A set of all the strings matching the given regex patterns.
        """
        data: Set[str] = set()

        for node in self.targets:
            maxtract.verbose_print(f"Parsing html {node.url}")
            found = set(re.findall(self.pattern, self._clean_html(node.html)))
            data = data.union(found)

        return data

    @staticmethod
    def _clean_html(html: str) -> str:
        """Removes unwanted tag types from the html.

        :param html: The html to be cleaned.
        :return: The cleaned html content.
        """
        soup = BeautifulSoup(html, "html5lib")

        for item in soup.find_all(Extractor._IGNORE):
            item.decompose()

        return soup.get_text("\n")
