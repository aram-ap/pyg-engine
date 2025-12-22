from setuptools import setup, find_packages
from setuptools_rust import Binding, RustExtension

with open("README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()

setup(
    name="pyg-engine",
    version="1.2.0",
    author="Aram Aprahamian",
    author_email="",
    description="A Python game engine with physics, rendering, and input systems",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="",
    packages=find_packages(where="src"),
    package_dir={"": "src"},
    rust_extensions=[
        RustExtension(
            "pyg_engine.pyg_engine_native",
            binding=Binding.PyO3,
            path="Cargo.toml",
        ),
    ],
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
        "Programming Language :: Rust",
        "Topic :: Games/Entertainment",
        "Topic :: Multimedia :: Graphics",
        "Topic :: Software Development :: Libraries :: Python Modules",
    ],
    python_requires=">=3.8",
    install_requires=[ ],
    setup_requires=["setuptools-rust>=1.9.0"],
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
        "pyg_engine": ["etc/*.png"],
    },
    entry_points={
        "console_scripts": [
            "pyg-engine=pyg_engine.utilities.cli:main",
        ],
    },
    zip_safe=False,
)
