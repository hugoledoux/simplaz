from setuptools import setup
import toml

with open("README.md", "r") as fh:
    long_description = fh.read()

with open("Cargo.toml") as fp:
    f = toml.load(fp)
    version = f["package"]["version"]
    description = f["package"]["description"]



setup(
    name="simplaz",
    version=version,
    description=description,
    long_description=long_description,
    long_description_content_type="text/x-md",
    url='https://github.com/hugoledoux/simplaz',
    author='Hugo Ledoux',
    author_email='h.ledoux@tudelft.nl',
    classifiers=[
        "License :: OSI Approved :: MIT License",
    ],
    packages=["simplaz"],
    python_requires='>=3.6',
)



