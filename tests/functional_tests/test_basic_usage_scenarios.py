#!/usr/bin/env python3
"""
# Suggested repo path: tests/functional_tests/test_basic_usage_scenarios.py

End-to-end smoke test for Fractal Amadeus

This script tests basic connectivity with Claude 3.5 Haiku API and
verifies that the system can properly respond as Amadeus Kurisu.
"""

from __future__ import annotations

import os
import sys
import json
from typing import Any, Dict, List, Optional, Protocol, TypedDict, cast
import requests
from requests.models import Response
import pytest
from dotenv import load_dotenv
from icecream import ic
from typeguard import typechecked
import icontract

# Fix the import path: amadedeus_client -> amadeus_client
from fractal_amadeus.core.amadeus_client import AmadeusProtocol

# Load API key from .env file
load_dotenv()
CLAUDE_API_KEY: Optional[str] = os.getenv("CLAUDE_API_KEY")

if not CLAUDE_API_KEY:
    print("Error: CLAUDE_API_KEY not found in environment variables. Please set it in .env file.")
    sys.exit(1)

# Claude API configuration
API_URL: str = "https://api.anthropic.com/v1/messages"
HEADERS: Dict[str, str] = {
    "Content-Type": "application/json",
    "anthropic-version": "2023-06-01",
    "x-api-key": CLAUDE_API_KEY
}

# Type definitions for API response
class MessageContent(TypedDict):
    type: str
    text: str

class ClaudeAPIResponse(TypedDict):
    id: str
    type: str
    role: str
    content: List[MessageContent]
    model: str
    stop_reason: str
    stop_sequence: Optional[str]
    usage: Dict[str, int]


@icontract.ensure(lambda result: len(result) > 0, "Preprompt must not be empty")
@icontract.ensure(lambda result: "Kurisu" in result, "Preprompt must contain Kurisu character information")
@typechecked
def load_preprompt() -> str:
    """
    Load the Kurisu character prompt from the file system.

    Returns:
        str: The content of the latest_preprompt.md file.

    Raises:
        FileNotFoundError: If the preprompt file cannot be found.
    """
    try:
        with open("docs/latest_preprompt.md", "r") as f:
            return f.read()
    except FileNotFoundError:
        print("Error: Could not find latest_preprompt.md")
        sys.exit(1)


@pytest.fixture(scope="session")
def http_session() -> requests.Session:
    """
    Create and provide a requests.Session object for the test session.

    Returns:
        requests.Session: A session object for making HTTP requests.
    """
    session: requests.Session = requests.Session()
    yield session
    session.close()  # cleanup after all tests are done


class Amadeus:
    """
    Amadeus client implementation for testing the Claude API with the Kurisu persona.
    """

    @icontract.require(lambda http_session: http_session is not None, "HTTP session cannot be None")
    @typechecked
    def __init__(self, http_session: requests.Session) -> None:
        """
        Initialize the Amadeus client with an HTTP session.

        Args:
            http_session: A requests.Session object for making HTTP requests.
        """
        # Things related to the quest:
        self.latest_request_api_url: Optional[str] = None
        self.latest_request_headers: Optional[Dict[str, str]] = None
        self.latest_request_json: Optional[Dict[str, Any]] = None

        # Things related to the response
        self.latest_message_content: Optional[str] = None
        self.latest_response: Optional[Response] = None

        # HTTP session
        self.http_session: requests.Session = http_session

    @icontract.require(lambda text: text, "Input text cannot be empty")
    @icontract.ensure(lambda result: result, "Response cannot be empty")
    @typechecked
    def test(self, text: str) -> str:
        """
        Run a basic smoke test to verify Claude API connectivity and character response.

        Args:
            text: The text to send to the Claude API.

        Returns:
            str: The response message content from the Claude API.

        Raises:
            requests.exceptions.RequestException: If the API request fails.
        """
        print("🔬 Initiating Fractal Amadeus e2e smoke test sequence...")
        print("🧪 Testing connection to Claude 3.5 Haiku API...")

        preprompt: str = load_preprompt()

        # Prepare the API request
        payload: Dict[str, Any] = {
            "model": "claude-3-haiku-20240307",
            "temperature": 0,             # Full determinism – always choose the most likely token
            "top_p": 0.9,                 # Sample only from the top 90% probability mass (slightly trims edge cases)
            "stop_sequences": ["User:"],  # Stops at a logical boundary – useful if you later auto-parse response blocks
            "max_tokens": 1000,           # Reasonable ceiling to ensure long enough replies but avoid overrun

        "messages": [
                {
                    "role": "user",
                    "content": f"{preprompt}\n\n{text}"
                }
            ]
        }

        # Send the request to Claude API
        self.latest_request_api_url = API_URL
        self.latest_request_headers = HEADERS
        self.latest_request_json = payload

        response: Response = requests.post(API_URL, headers=HEADERS, json=payload)
        response.raise_for_status()
        self.latest_response = response

        # Parse the response
        result: ClaudeAPIResponse = cast(ClaudeAPIResponse, response.json())
        self.latest_message_content = result["content"][0]["text"]

        # Display the response
        print("\n===== AMADEUS SYSTEM RESPONSE =====\n")
        print(self.latest_message_content)
        print("\n===================================\n")

        # Return it now:
        return self.latest_message_content

    @icontract.require(lambda self: self.latest_message_content is not None,
                     "No message content available. Run test() first.")
    @icontract.require(lambda expected_elements: len(expected_elements) > 0,
                     "Expected elements list cannot be empty")
    @typechecked
    def raise_for_some_elements_not_returned(self, expected_elements: List[str]) -> None:
        """
        Check if the expected elements are present in the latest message content.

        Args:
            expected_elements: A list of strings that are expected to be in the latest message content.

        Raises:
            ValueError: If no message content is available.
            AssertionError: If any expected elements are not found in the content.
        """
        if self.latest_message_content is None:
            raise ValueError("No message content available. Run test() first.")

        missing_elements: List[str] = [elem for elem in expected_elements if elem not in self.latest_message_content]
        assert missing_elements == [], f"Missing elements: {missing_elements}"


@pytest.fixture
def amadeus(http_session: requests.Session) -> Amadeus:
    """
    Fixture providing an Amadeus client instance for testing.

    Args:
        http_session: The HTTP session fixture.

    Returns:
        Amadeus: An instance of the Amadeus client.
    """
    return Amadeus(http_session)


@icontract.require(lambda amadeus: amadeus is not None, "Amadeus client cannot be None")
@typechecked
def test_okabe_wants_something_more_healthy_so_he_asks_kurisu(amadeus: Amadeus) -> None:
    """
    Test that Okabe can ask Kurisu for healthy food alternatives.

    Args:
        amadeus: The Amadeus client fixture.
    """
    response: str = amadeus.test(
        "The user, Okabe, opens the phone app and asks Amadeuis Kurisu about healthy alternatives to cup noodles."
    )
    assert "Does this help provide some healthier meal ideas" in response


@icontract.require(lambda amadeus: amadeus is not None, "Amadeus client cannot be None")
@typechecked
def test_okabe_greets_kurisu(amadeus: Amadeus) -> None:
    """
    Test that Okabe can greet Kurisu and get a proper response.

    Args:
        amadeus: The Amadeus client fixture.
    """
    response: str = amadeus.test(
        "Hello there. This is Rintaro Okabe, also known as Hououin Kyouma! Is this the famous Amadeus system I've heard about? Christina, can you hear me?"
    )
    amadeus.raise_for_some_elements_not_returned(["Kurisu:", "System boot", "["])
