"""
Tests of the RacHL7 parser
"""
from pytest import fixture

import rachl7


@fixture
def hl7document():
    """
    A document we can parse to produce a parse tree
    """
    return "\x02MSH|ono|\r\n\r"


def test_parse(hl7document):
    """
    Is the expression parseable?
    """
    assert rachl7.parse("22")
    assert rachl7.parse("(22)")
    assert rachl7.parse("((((22))))")
    assert not rachl7.parse("((22)")
