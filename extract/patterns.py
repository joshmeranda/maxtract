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
