"""
# Suggested repo path: tests/unit_tests/test_okabe_message_flow.py

Unit tests for the Okabe message flow in the Fractal Amadeus system.
"""

from __future__ import annotations
import pytest
from typeguard import typechecked
import icontract
from typing import List, Optional, Protocol, Final, cast

from fractal_amadeus.core import load_preprompt, get_other_name_for_okabe


class AmadeusProtocol(Protocol):
    """Protocol defining the interface for Amadeus test clients."""

    latest_message_content: Optional[str]
    latest_request_api_url: Optional[str]

    def test(self, text: str) -> str:
        """Test the Amadeus system with the given text."""
        ...

    def raise_for_some_elements_not_returned(self, expected_elements: List[str]) -> None:
        """Check if expected elements are present in the response."""
        ...


@icontract.require(lambda: True, "Preprompt must be accessible")
@typechecked
def test_can_load_preprompt() -> None:
    """
    Test that we can load the preprompt file.

    This test verifies that the preprompt contains expected character information
    about Kurisu's scientific background.
    """
    content = load_preprompt()
    assert "Child prodigy neuroscientist" in content, "Preprompt must contain Kurisu's scientific background"


@icontract.require(lambda: True, "Function must be callable")
@typechecked
def test_okabe_calls_himself_Hououin_Kyouma() -> None:
    """
    Test that Okabe's alter ego name is correct.

    This test verifies that the get_other_name_for_okabe function
    returns the expected alter ego name.
    """
    name = get_other_name_for_okabe()
    assert name == "Hououin Kyouma", "Okabe's alter ego name must be exactly 'Hououin Kyouma'"


@typechecked
class DummyAmadeus(AmadeusProtocol):
    """A dummy implementation of the Amadeus protocol for testing."""

    FIXED_RESPONSE: Final[str] = "Does this help provide some healthier meal ideas"

    @icontract.ensure(lambda self: self.latest_message_content == self.FIXED_RESPONSE,
                     "Message content must be initialized to the fixed response")
    @typechecked
    def __init__(self) -> None:
        """Initialize the dummy Amadeus with default values."""
        self.latest_request_api_url: Optional[str] = None
        self.latest_message_content: str = self.FIXED_RESPONSE

    @icontract.ensure(lambda result, self: result == self.FIXED_RESPONSE,
                     "Response must match the fixed response")
    @typechecked
    def test(self, text: str) -> str:
        """
        Test the Amadeus system with the given text.

        Args:
            text: The text to send to Amadeus

        Returns:
            A fixed response string
        """
        return self.FIXED_RESPONSE

    @icontract.require(lambda self: self.latest_message_content is not None,
                     "Message content must be available")
    @icontract.require(lambda expected_elements: len(expected_elements) > 0,
                     "Expected elements list cannot be empty")
    @typechecked
    def raise_for_some_elements_not_returned(self, expected_elements: List[str]) -> None:
        """
        Check if expected elements are present in the response.

        Args:
            expected_elements: List of strings that should be in the response

        Raises:
            AssertionError: If any elements are missing
        """
        missing_elements: List[str] = [elem for elem in expected_elements if elem not in self.latest_message_content]
        assert missing_elements == [], f"Missing elements: {missing_elements}"


@pytest.fixture
def amadeus() -> DummyAmadeus:
    """
    Fixture providing a DummyAmadeus instance.

    Returns:
        A new DummyAmadeus instance
    """
    return DummyAmadeus()


@icontract.require(lambda amadeus: amadeus is not None, "Amadeus instance must be provided")
@typechecked
def test_dummy_amadeus_passes_tests_with_flying_colors(amadeus: DummyAmadeus) -> None:
    """
    Test that the dummy Amadeus returns the expected response.

    Args:
        amadeus: The DummyAmadeus instance to test
    """
    response: str = amadeus.test(
        "The user, Okabe, opens the phone app and asks Amadeus Kurisu about healthy alternatives to cup noodles."
    )
    assert "Does this help provide some healthier meal ideas" in response, "Response must contain the expected text"


@icontract.require(lambda amadeus: amadeus is not None, "Amadeus instance must be provided")
@icontract.ensure(lambda amadeus: amadeus.latest_request_api_url is None,
                 "API URL must remain None after test")
@typechecked
def test_amadeus_failing_with_bad_test(amadeus: DummyAmadeus) -> None:
    """
    Test that the dummy Amadeus has the expected API URL.

    Args:
        amadeus: The DummyAmadeus instance to test
    """
    response: str = amadeus.test(
        "The user, Okabe, opens the phone app and asks Amadeus Kurisu about healthy alternatives to cup noodles."
    )
    assert amadeus.latest_request_api_url is None, "API URL should be None for DummyAmadeus"


@icontract.require(lambda amadeus: amadeus is not None, "Amadeus instance must be provided")
@typechecked
def test_amadeus_failing_with_bad_expected_values(amadeus: DummyAmadeus) -> None:
    """
    Test the behavior when checking for elements that aren't present.

    Args:
        amadeus: The DummyAmadeus instance to test
    """
    amadeus.test(
        "The user, Okabe, opens the phone app and asks Amadeus Kurisu about healthy alternatives to cup noodles."
    )
    # This test will fail because "Doggoes" is not in the response
    with pytest.raises(AssertionError):
        amadeus.raise_for_some_elements_not_returned(["Kurisu:", "System boot", "[", "Doggoes"])


@icontract.require(lambda amadeus: amadeus is not None, "Amadeus instance must be provided")
@typechecked
def test_amadeus_fails_with_some_expected_values_not_present(amadeus: DummyAmadeus) -> None:
    """
    Test the behavior when checking for elements that are present.

    Args:
        amadeus: The DummyAmadeus instance to test
    """
    amadeus.test(
        "Hello there. This is Rintaro Okabe, also known as Hououin Kyouma! Is this the famous Amadeus system I've heard about? Christina, can you hear me?"
    )
    # This will fail because our dummy response doesn't contain these elements
    with pytest.raises(AssertionError):
        amadeus.raise_for_some_elements_not_returned(["Kurisu:", "System boot", "["])
