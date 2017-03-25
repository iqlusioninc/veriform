#!/usr/bin/env python
# -*- coding: utf-8 -*-

from setuptools import setup

requirements = []
test_requirements = []

setup(
    name="zser",
    version="0.0.0",
    description="zcred serialization format",
    long_description="A protobuf-inspired minimalistic serialization format with cryptographic authentication",
    author="Tony Arcieri",
    author_email="bascule@gmail.com",
    url="https://github.com/anupsv/zser",
    packages=["zser"],
    package_dir={"zser": "zser"},
    include_package_data=True,
    install_requires=[],
    license="MIT license",
    zip_safe=False,
    keywords=["authentication", "cryptography", "security", "serialization", "merkle"],
    classifiers=[
        "Development Status :: 2 - Pre-Alpha",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Natural Language :: English",
    ],
    test_suite="tests",
    tests_require=[]
)
