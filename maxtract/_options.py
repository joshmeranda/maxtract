import argparse


def _parse_args() -> argparse.Namespace:
    """Parse program arguments."""
    maxtract = argparse.ArgumentParser(prog="maxtract", add_help=True,
                                       description="Find all links contained in a website, and"
                                                   "extract data")

    sub_commands = maxtract.add_subparsers(title="Commands", dest="command", required=True)

    _parse_mapper(sub_commands)
    _parse_extract(sub_commands)

    options = maxtract.parse_args()

    # ensure that a data type to extract is specified along with extract sub-command
    if options.command == "extract" and not options.email and not options.phone \
            and not options.regex:
        maxtract.error(
            "No data type specified, please specify one of '--email', '--phone', or '--regex'")

    return maxtract.parse_args()


def _parse_mapper(parent: argparse._SubParsersAction):
    """Add the mapper sub command.

    Params:
        parent (argparse._SubParsersAction): The parser to which the sub command is to be added.
    """
    mapper = parent.add_parser("mapper")

    mapper.add_argument("url", action="store", type=str,
                        help="The root url to start the map")
    mapper.add_argument("--travel", action="store_true",
                        help="Specify whether to follow external links")
    mapper.add_argument("-d", "--depth", action="store", type=int, required=False,
                        help="The greatest depth of links to follow")
    mapper.add_argument("--copy", action="store_true",
                        help="Specify whether to create a local copy of the website")
    mapper.add_argument("-p", "--path", action="store", default=".", type=str,
                        help="The path to store the local copy, defaults to '.'")


def _parse_extract(parent: argparse._SubParsersAction):
    """Add the extract sub command.

    Params:
        parent (argparse._SubParsersAction): The parser to which the sub command is to be added.
    """
    extract = parent.add_parser("extract")

    extract_type = extract.add_argument_group(title="Extraction Types",
                                              description="The type if information to extract.")
    extract_type.add_argument("--email", action="store_true", help="Extract emails")
    extract_type.add_argument("--phone", action="store_true", help="Extract phone numbers")
    extract_type.add_argument("--regex", action="store", type=str,
                              help="Extract using the given regex pattern")

    extract.add_argument("url", action="store", nargs="+",
                         help="List of files or urls to pars for information")
