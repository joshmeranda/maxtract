"""Hold exception class definitions."""


class NodeError(Exception):
    """Error instantiating a new Node instance."""

    def __init__(self, url):
        self.url = url
