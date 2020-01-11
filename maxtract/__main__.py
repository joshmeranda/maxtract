#!/usr/bin/env python3
from maxtract._options import _parse_args
from maxtract._extract import run_extract
from maxtract._mapper import run_mapper
from argparse import Namespace


def main():
    options: Namespace = _parse_args()

    if options.command == "mapper":
        run_mapper(options)
    elif options.command == "extract":
        run_extract(options)


if __name__ == "__main__":
    main()
