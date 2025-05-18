"""
# Suggested repo path: fractal_amadeus/core/amadeus_client/__init__.py

AmadeusClientProtocol module defining the core interfaces for interacting with Amadeus.
"""

from __future__ import annotations
from typing import Protocol, List, Optional, runtime_checkable
from typeguard import typechecked
import icontract


@runtime_checkable
class AmadeusProtocol(Protocol):
    """Protocol defining the interface for Amadeus clients."""

    latest_message_content: Optional[str]
    latest_request_api_url: Optional[str]

    def test(self, text: str) -> str:
        """
        Tests the Amadeus system with the given input text.

        Args:
            text: The text to send to the Amadeus system

        Returns:
            The response from the Amadeus system
        """
        ...

    def raise_for_some_elements_not_returned(self, expected_elements: List[str]) -> None:
        """
        Raises an assertion error if any expected elements are not present in the latest message.

        Args:
            expected_elements: List of strings that should appear in the latest response

        Raises:
            AssertionError: If any elements are missing from the response
        """
        ...


@runtime_checkable
class AmadeusClientProtocol(Protocol):
    """Protocol defining the interface for Amadeus clients."""
    pass


@icontract.invariant(lambda self: hasattr(self, '_initialized'),
                     "Client must be properly initialized")
@typechecked
class AmadeusKurisuClient(AmadeusClientProtocol):
    @typechecked
    def __init__(self) -> None:
        """Initialize a new Amadeus Kurisu client."""
        self._initialized = True
