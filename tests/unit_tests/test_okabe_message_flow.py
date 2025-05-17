# from fractal_amadeus.core.amadeus_client import AmadeusKurisuClient
#
# def test_okabe_gets_kurisu_response(monkeypatch):
#     # Arrange
#     test_message = "Hello, this is Okabe Rintaro. Christina, are you there?"
#     expected_keywords = ["Kurisu:", "System boot", "["]
#
#     def fake_call_claude(prompt):
#         return """
#         [System boot sequence initializing...]
#         [Scene: Amadeus Lab - Kurisu appears on the holoscreen]
#         Kurisu: You're calling me *that* nickname again? How persistent...
#         """
#
#     # Monkeypatch Claude API call
#     monkeypatch.setattr(AmadeusKurisuClient, "send_message", lambda self, msg: fake_call_claude(msg))
#
#     # Act
#     client = AmadeusKurisuClient()
#     response = client.send_message(test_message)
#
#     # Assert
#     for keyword in expected_keywords:
#         assert keyword in response, f"Expected keyword '{keyword}' not found in response."
