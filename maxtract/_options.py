import argparse
from sys import stdout


def _parse_args() -> argparse.Namespace:
    """Parse program arguments."""
    maxtract = argparse.ArgumentParser(prog="maxtract", add_help=True,
                                       description="Extract data from a website")

    mapper = maxtract.add_argument_group(title="mapper options",
                                               description="options which modify the")
    maxtract.add_argument("url", action="store", type=str, help="the root url from which to extract data")
    maxtract.add_argument("--travel", action="store_true", help="specify whether to follow external links")
    maxtract.add_argument("-d", "--depth", action="store", type=int, help="the greatest depth of links to follow")
    maxtract.add_argument("--local", action="store", default=".", type=str,
                          help="if specified a local copy of the retrieved data is created at the given path")
    maxtract.add_argument("-f", "--file", action="store", default=stdout, type=argparse.FileType("w"),
                          help="the file into which the output is stored")

    extract_type = maxtract.add_argument_group(title="Extraction Types",
                                               description="The type if information to extract")
    extract_type.add_argument("--email", action="store_true", help="Extract emails")
    extract_type.add_argument("--phone", action="store_true", help="Extract phone numbers")
    extract_type.add_argument("--regex", action="store", type=str,
                              help="Extract using the given regex pattern")

    options = maxtract.parse_args()

    # ensure that a data type to extract is specified along with extract sub-command
    if options == "extract" and not options.email and not options.phone \
            and not options.regex:
        maxtract.error(
            "No data type specified, please specify one of '--email', '--phone', or '--regex'")

    return maxtract.parse_args()
