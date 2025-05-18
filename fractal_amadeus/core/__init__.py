"""
# Suggested repo path: fractal_amadeus/core/__init__.py

Core functionality for the Fractal Amadeus system.
"""

from __future__ import annotations
import icontract
from typeguard import typechecked
from typing import Callable


@icontract.require(lambda: True, "Always valid")  # Basic contract to demonstrate structure
@icontract.ensure(lambda result: 10 <= len(result) <= 100_000,
                 "Preprompt length must be between 10 and 100,000 characters")
@icontract.ensure(lambda result: "Kurisu" in result,
                 "Preprompt must contain Kurisu character information")
@typechecked
def load_preprompt() -> str:
    """
    Load the latest preprompt file for Amadeus Kurisu.

    Returns:
        The contents of the preprompt file as a string

    Ensures:
        - The length of the preprompt is between 10 and 100,000 characters
        - The preprompt contains information about Kurisu

    Raises:
        FileNotFoundError: If the preprompt file cannot be found
    """
    with open("docs/latest_preprompt.md", "r") as f:
        content = f.read()
        return content


@icontract.ensure(lambda result: result == "Hououin Kyouma",
                 "Result must be exactly 'Hououin Kyouma'")
@typechecked
def get_other_name_for_okabe() -> str:
    """
    Get Okabe's self-given alias.

    Returns:
        The string "Hououin Kyouma"

    Ensures:
        The result is exactly "Hououin Kyouma"
    """
    return "Hououin Kyouma"
