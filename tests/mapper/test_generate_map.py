from unittest import TestCase
from tests import constants
from mapper import Node
from mapper.utils import generate_map


class TestMapperGenerateMap(TestCase):
    def __init__(self, *args):
        super().__init__(*args)
        self.maxDiff = None

    def __assertElements(self, expected, actual):
        for e, a in zip(expected, actual):
            self.assertEqual(e, a)

    def test_linear(self):
        linear_dir = "file://" + constants.RESOURCE_DIR + "/linear"
        expected = [Node(f"{linear_dir}/index.html"),
                    Node(f"{linear_dir}/child_00.html"),
                    Node(f"{linear_dir}/child_01.html"),
                    Node(f"{linear_dir}/dir/child_10.html")]

        actual = generate_map(f"{linear_dir}/index.html")

        self.assertEqual(expected.sort(), actual.sort())

    def test_recursive(self):
        recur_dir = "file://" + constants.RESOURCE_DIR + "/recursive"
        expected = [Node(f"{recur_dir}/index.html"),
                    Node(f"{recur_dir}/child_00.html"),
                    Node(f"{recur_dir}/child_01.html"),
                    Node(f"{recur_dir}/child_10.html")]

        actual = generate_map(
            f"file://{constants.RESOURCE_DIR}/recursive/index.html")

        self.assertEqual(expected.sort(), actual.sort())

    def test_limit_depth_zero(self):
        linear_dir = "file://" + constants.RESOURCE_DIR + "/linear"
        expected = [Node(f"{linear_dir}/index.html")]
        actual = generate_map(f"{linear_dir}/index.html", depth=0)
        self.assertEqual(expected, actual)

    def test_limit_depth_one(self):
        linear_dir = "file://" + constants.RESOURCE_DIR + "/linear"
        expected = [Node(f"{linear_dir}/index.html"),
                    Node(f"{linear_dir}/child_00.html"),
                    Node(f"{linear_dir}/child_01.html")]

        actual = generate_map(f"{linear_dir}/index.html", depth=1)
        self.assertEqual(expected.sort(), actual.sort())

    def test_ignore_external(self):
        # Will also test for normalizing link with different schemas.
        non_local_dir = "file://" + constants.RESOURCE_DIR + "/non_local"
        expected = [Node(f"{non_local_dir}/index.html")]
        actual = generate_map(f"{non_local_dir}/index.html", local=True)
        self.assertEqual(expected, actual)
