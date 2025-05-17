#!/usr/bin/env python3
"""
tests/e2e_smoke_test.py - End-to-end smoke test for Fractal Amadeus

This script tests basic connectivity with Claude 3.5 Haiku API and
verifies that the system can properly respond as Amadeus Kurisu.
"""

import os
import sys
import json
import requests
from dotenv import load_dotenv

# Load API key from .env file
load_dotenv()
CLAUDE_API_KEY = os.getenv("CLAUDE_API_KEY")

if not CLAUDE_API_KEY:
    print("Error: CLAUDE_API_KEY not found in environment variables. Please set it in .env file.")
    sys.exit(1)

# Claude API configuration
API_URL = "https://api.anthropic.com/v1/messages"
HEADERS = {
    "Content-Type": "application/json",
    "anthropic-version": "2023-06-01",
    "x-api-key": CLAUDE_API_KEY
}

# Load the Kurisu character prompt
def load_preprompt():
    try:
        with open("docs/latest_preprompt.md", "r") as f:
            return f.read()
    except FileNotFoundError:
        print("Error: Could not find latest_preprompt.md")
        sys.exit(1)


def test_run_smoke_test():
    """Run a basic smoke test to verify Claude API connectivity and character response"""
    print("🔬 Initiating Fractal Amadeus e2e smoke test sequence...")
    print("🧪 Testing connection to Claude 3.5 Haiku API...")

    preprompt = load_preprompt()

    # Simple test message in the style of Steins;Gate
    test_message = "Hello there. This is Rintaro Okabe, also known as Hououin Kyouma! Is this the famous Amadeus system I've heard about? Christina, can you hear me?"

    # Prepare the API request
    payload = {
        "model": "claude-3-haiku-20240307",
        "temperature": 0,             # Full determinism – always choose the most likely token
        "top_p": 0.9,                 # Sample only from the top 90% probability mass (slightly trims edge cases)
        "stop_sequences": ["User:"],  # Stops at a logical boundary – useful if you later auto-parse response blocks
        "max_tokens": 1000,           # Reasonable ceiling to ensure long enough replies but avoid overrun
        "messages": [
            {
                "role": "user",
                "content": f"{preprompt}\n\n{test_message}"
            }
        ]
    }

    # Send the request to Claude API
    print("📡 Establishing connection to Amadeus system...")
    response = requests.post(API_URL, headers=HEADERS, json=payload)
    response.raise_for_status()

    # Parse the response
    result = response.json()
    message_content = result["content"][0]["text"]

    # Display the response
    print("\n===== AMADEUS SYSTEM RESPONSE =====\n")
    print(message_content)
    print("\n===================================\n")

    # Check if the response contains expected elements
    expected_elements = ["Kurisu:", "System boot", "["]
    missing_elements = [elem for elem in expected_elements if elem not in message_content]
    assert missing_elements == []


if __name__ == "__main__":
    success = test_run_smoke_test()
    sys.exit(0 if success else 1)
