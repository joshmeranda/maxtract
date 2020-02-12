from mapper.utils import normalize_link
from mapper.utils import generate_map
from mapper.utils import make_local_copy
from mapper import Node
from tests import constants
from unittest import TestCase
from urllib import parse
import os


class TestUtilNormalize(TestCase):
    index_html = f"file://{constants.RESOURCE_DIR}/linear/index.html"
    file_parent = "file:///path/to/page/index.html"
    url_parent = "https://domain.com/path/index.html"

    def test_normalize_file_plain(self):
        child = "file.txt"
        self.assertEqual("file:///path/to/page/file.txt",
                         normalize_link(TestUtilNormalize.file_parent, child))

    def test_normalize_file_extras(self):
        extras = ";params?query#fragment"
        parent = TestUtilNormalize.file_parent + extras
        child = "file.txt" + extras
        self.assertEqual("file:///path/to/page/file.txt" + extras,
                         normalize_link(parent, child))

    def test_normalize_file_relative(self):
        child = "../file.txt"
        self.assertEqual("file:///path/to/file.txt",
                         normalize_link(TestUtilNormalize.file_parent, child), )

    def test_normalize_file_absolute(self):
        child = TestUtilNormalize.index_html
        self.assertEqual(TestUtilNormalize.index_html,
                         normalize_link(TestUtilNormalize.file_parent, child))

    def test_normalize_url_plain(self):
        child = "file.txt"
        self.assertEqual("https://domain.com/path/file.txt",
                         normalize_link(TestUtilNormalize.url_parent, child))

    def test_normalize_url_extras(self):
        extras = ";params?query#fragment"
        parent = TestUtilNormalize.url_parent + extras
        child = "file.txt" + extras
        self.assertEqual("https://domain.com/path/file.txt" + extras,
                         normalize_link(parent, child))

    def test_normalize_url_relative(self):
        child = "../file.txt"
        self.assertEqual("https://domain.com/file.txt",
                         normalize_link(TestUtilNormalize.url_parent, child))

    def test_normalize_url_absolute(self):
        child = "/index.html"
        self.assertEqual("https://domain.com/index.html",
                         normalize_link(TestUtilNormalize.url_parent, child))


class TestUtilGenerateMap(TestCase):
    def __init__(self, *args):
        super().__init__(*args)
        self.maxDiff = None

    def __assertElements(self, expected, actual):
        for e, a in zip(expected, actual):
            self.assertEqual(e, a)

    def test_linear(self):
        linear_dir = "file://" + constants.RESOURCE_DIR + "/linear"
        expected = {Node(f"{linear_dir}/index.html"),
                    Node(f"{linear_dir}/child_00.html"),
                    Node(f"{linear_dir}/child_01.html"),
                    Node(f"{linear_dir}/dir/child_10.html")}

        actual = generate_map(f"{linear_dir}/index.html", file=None)

        self.assertEqual(expected, actual)

    def test_recursive(self):
        recur_dir = "file://" + constants.RESOURCE_DIR + "/recursive"
        expected = {Node(f"{recur_dir}/index.html"),
                    Node(f"{recur_dir}/child_00.html"),
                    Node(f"{recur_dir}/child_01.html"),
                    Node(f"{recur_dir}/child_10.html")}

        actual = generate_map(f"file://{constants.RESOURCE_DIR}/recursive/index.html", file=None)

        self.assertEqual(expected, actual)

    def test_limit_depth_zero(self):
        linear_dir = "file://" + constants.RESOURCE_DIR + "/linear"
        expected = {Node(f"{linear_dir}/index.html")}
        actual = generate_map(f"{linear_dir}/index.html", depth=0, file=None)
        self.assertEqual(expected, actual)

    def test_limit_depth_one(self):
        linear_dir = "file://" + constants.RESOURCE_DIR + "/linear"
        expected = {Node(f"{linear_dir}/index.html"),
                    Node(f"{linear_dir}/child_00.html"),
                    Node(f"{linear_dir}/child_01.html")}

        actual = generate_map(f"{linear_dir}/index.html", depth=1, file=None)
        self.assertEqual(expected, actual)

    def test_ignore_external(self):
        # Will also test for normalizing link with different schemas.
        non_local_dir = "file://" + constants.RESOURCE_DIR + "/non_local"
        expected = {Node(f"{non_local_dir}/index.html")}
        actual = generate_map(f"{non_local_dir}/index.html", local=True, file=None)
        self.assertEqual(expected, actual)

    def test_write_to_file(self):
        linear_dir = "file://" + constants.RESOURCE_DIR + "/linear"
        expected = {f"{linear_dir}/index.html\n",
                    f"{linear_dir}/child_00.html\n",
                    f"{linear_dir}/child_01.html\n",
                    f"{linear_dir}/dir/child_10.html\n"}
        out_path = f"{constants.TESTS_DIR}/out"

        try:
            with open(out_path, "w") as out:
                generate_map(f"{linear_dir}/index.html", local=True, file=out)

            with open(out_path, "r") as out:
                self.assertCountEqual(out.readlines(), expected)
        finally:  # ensure that the output file is always removed
            os.remove(out_path)


class TestMapperMakeLocalCopy(TestCase):
    temp_dir = os.path.join(constants.TESTS_DIR, "temp")
    lmap = generate_map("file://" + constants.RESOURCE_DIR + "/linear/index.html", file=None)

    def setUp(self):
        # if tmp dir exists remove and replace it
        if os.path.exists(TestMapperMakeLocalCopy.temp_dir):
            self.tearDown()
        os.mkdir(TestMapperMakeLocalCopy.temp_dir, 0o755)

    def tearDown(self):
        from shutil import rmtree
        rmtree(TestMapperMakeLocalCopy.temp_dir)

    def test_make_local_copy(self):
        make_local_copy(list(TestMapperMakeLocalCopy.lmap), TestMapperMakeLocalCopy.temp_dir)
        for node in TestMapperMakeLocalCopy.lmap:
            path: str = TestMapperMakeLocalCopy.temp_dir + parse.urlparse(node.url).path
            self.assertTrue(os.path.exists(path))
