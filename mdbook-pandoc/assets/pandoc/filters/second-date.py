#!/usr/bin/env python3

"""
Pandoc filter to change the date formatting in metadata from
whatever it is to whatever strftime format you specify. This
allows you to keep your source documents in one format, but
output to another format.
"""

from panflute import run_filter, MetaMap, Str
from dateutil import parser
from datetime import datetime


def make_pretty(elem, doc):
    if type(elem) == Str:
        dt = parser.parse(elem.text)
        pretty_date = dt.strftime("%B %d, %Y")
        return Str(text=pretty_date)


def second_date(elem, doc):
    if isinstance(elem, MetaMap) and "date" in elem.content:
        elem.content["date"].walk(make_pretty, doc)


def main(doc=None):
    return run_filter(second_date, doc=doc)


if __name__ == "__main__":
    main()
