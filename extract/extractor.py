"""Define classes for extraction abstraction."""
import re
from typing import List
from mapper.node import Node


class Extractor:
    """Extracts data from the urls provided.

    Attributes:
        targets (lis1t): The list of urls from which to extract data.
        pattern (str): The regular expression to use when extracting data.

    Params:
        targets (list): The list of urls from which to extract data.
        pattern (str): The regular expression to use when extracting data.
    """
    def __init__(self, targets: List[Node], *patterns: str):
        self.targets = targets
        self.pattern = re.compile("|".join(patterns))

    def extract(self) -> List[str]:
        """Pull all text matching the instances regex pattern."""
        html = str()
        for node in self.targets:
            html += node.html
        return re.findall(self.pattern, html)
