from setuptools import setup
from setuptools_rust import Binding, RustExtension


__version__ = '0.0.0'


setup(
    description='The Rusty HL7 parser for Python',
    extras_require={
        'dev': [
            'tox',
            'pytest',
            'pytest-flakes',
        ],
        'deploy': [
            ## 'twine', ## not sure yet we'll see
        ]
    },
    install_requires=[],
    name='RacHL7',
    package_dir={'': 'src'},
    py_modules=['rachl7'],
    rust_extensions=[
        RustExtension('_rachl7', binding=Binding.PyO3),
    ],
    version=__version__,
    zip_safe=False,
)