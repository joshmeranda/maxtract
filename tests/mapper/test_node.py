from mapper import Node
from error import NodeError
from tests import constants
import unittest


class TestNode(unittest.TestCase):
    def test_node_children(self):
        linear_dir = "file://" + constants.RESOURCE_DIR + "/linear"
        test_node = Node(f"{linear_dir}/index.html")
        expected_links = {f"{linear_dir}/child_00.html", f"{linear_dir}/child_01.html"}
        self.assertEqual(expected_links, test_node.children)

    def test_node_no_children(self):
        test_node = Node(f"file://{constants.RESOURCE_DIR}/linear/child_01.html")
        self.assertEqual(set(), test_node.children)

    def test_node_barren(self):
        test_node = Node("file://" + constants.RESOURCE_DIR + "/linear/index.html", barren=True)
        self.assertEqual(set(), test_node.children)

    def test_node_error(self):
        self.assertRaises(NodeError, Node, "BAD_URL")
