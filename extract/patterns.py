"""Provides sample regex patterns to use for extracting data.

The following patterns are provided:
    EMAIL: Matches emails.
    PHONE_NUMBER: Matches phone numbers.
"""

EMAIL: str = "[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?" \
             "(?:\\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*"

PHONE_NUMBER: str = "(?:\\+?[0-9]{1,3}[\\s-]?)?" \
                    "(?:(?:\\(?[0-9]{3}\\)?)|(?:[0-9]{3}))" \
                    "(?:[\\s.-]?)" \
                    "(?:[0-9]{3})" \
                    "(?:[\\s.-]?)" \
                    "(?:[0-9]{4})"
