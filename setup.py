from setuptools import setup, find_packages

with open("README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()

setup(
    name="pyg-engine",
    version="1.0.0a2",
    author="Aram Aprahamian",
    author_email="",
    description="A Python game engine with physics, rendering, and input systems",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="",
    packages=["pyg_engine"],
    package_dir={"pyg_engine": "src"},
    classifiers=[
        "Development Status :: 4 - Beta",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Topic :: Games/Entertainment",
        "Topic :: Multimedia :: Graphics",
        "Topic :: Software Development :: Libraries :: Python Modules",
    ],
    python_requires=">=3.8",
    install_requires=[
        "pygame>=2.5.0",
        "pymunk>=6.4.0",
    ],
    extras_require={
        "dev": [
            "pytest>=7.0.0",
            "pytest-cov>=4.0.0",
            "black>=22.0.0",
            "flake8>=5.0.0",
        ],
    },
    include_package_data=True,
    package_data={
        "pyg_engine": ["*.py"],
    },
    entry_points={
        "console_scripts": [
            "pyg-engine=pyg_engine.cli:main",
        ],
    },
) 