#!/usr/bin/env python3
from distutils.core import setup


# use requirements.txt as the list of packages
with open('requirements.txt') as reqs:
    requirements = reqs.read().splitlines()

setup(name="WebMapper",
      version="0.1",
      description="Utility to map a website via accessible links, nd extract information.",
      author="Joshua Meranda",
      author_email="joshmeranda@gmail.com",
      packages=["maxtract"],
      requires=requirements
      )
