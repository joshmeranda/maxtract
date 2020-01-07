from extract import patterns
from unittest import TestCase
import re


class TestPatterns(TestCase):
    def test_EMAIL(self):
        content = ["email_00@email.com",
                   "This is some content with an email: email_01@email.com",
                   "This is a nested email (email_02@email.com)",
                   "Here we have several emails email_03@email.com email_04@email.com",
                   "Now we have a new format firstname.lastname@company.co"]
        emails = [["email_00@email.com"],
                  ["email_01@email.com"],
                  ["email_02@email.com"],
                  ["email_03@email.com", "email_04@email.com"],
                  ["firstname.lastname@company.co"]]
        pattern = re.compile(patterns.EMAIL)

        for content, expected in zip(content, emails):
            actual = re.findall(pattern, content)
            with self.subTest(content=content):
                self.assertEqual(expected, actual)

    def test_PHONE(self):
        content = ["Domestic 00: (000) 000-0000",
                   "Domestic 01: 000-000-0000",
                   "Domestic 02: 000.000.0000",
                   "Domestic 03: 000 000 0000",
                   "International 0: +1-000-000-0000"
                   "International 1: 1-000-000-0000"
                   ]
        numbers = ["(000) 000-0000",
                   "000-000-0000",
                   "000.000.0000",
                   "000 000 0000",
                   "+1-000-000-0000",
                   "1-000-000-0000"]
        pattern = re.compile(patterns.PHONE_NUMBER)

        for content, expected in zip(content, numbers):
            actual = re.search(pattern, content).group(0)
            with self.subTest(content=content):
                self.assertEqual(expected, actual)