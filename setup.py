from setuptools import setup


__version__ = '0.0.0'


kw = dict(
)

setup(
    name='RacHL7',
    description='The Rusty HL7 parser for Python',
    version=__version__,
    package_dir={'': 'src'},
    py_modules=['rachl7'],
    install_requires=[],
    extras_require={
        'dev': [
            'maturin',
            'tox',
            'wheel',
            'pytest',
            'pytest-flakes',
        ],
        'deploy': [
            # 'twine', ## not sure yet we'll see
        ]
    },
    zip_safe=False,
)