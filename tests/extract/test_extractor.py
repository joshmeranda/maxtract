from extract import Extractor
from extract import patterns
from unittest import TestCase
from tests import constants
from mapper.utils import generate_map

TARGET_DIR = "file://" + constants.RESOURCE_DIR + "/linear"


class TestExtractor(TestCase):
    lmap = generate_map("file://" + constants.RESOURCE_DIR + "/linear/index.html", file=None)

    def test_extractor_email(self):
        extractor = Extractor(TestExtractor.lmap, patterns.EMAIL)
        expected = ["test.email@test.com",
                    "test.email@test.org",
                    "test.email@test.edu",
                    "test.email@test.co"]
        actual = extractor.extract()
        self.assertCountEqual(expected, actual)

    def test_extract_email_and_phone(self):
        extractor = Extractor(TestExtractor.lmap, patterns.EMAIL, patterns.PHONE_NUMBER)
        expected = ["test.email@test.com",
                    "test.email@test.org",
                    "test.email@test.edu",
                    "test.email@test.co",
                    "000-000-0000",
                    "(000) 000-0000",
                    "+1(000) 000-0000",
                    "000.000.0000"]
        actual = extractor.extract()
        self.assertCountEqual(expected, actual)