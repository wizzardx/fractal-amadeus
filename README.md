# 🧠 Fractal Amadeus: The Kurisu Emulator for Scientific Dialogue

> A lightweight, prompt-powered simulation of Amadeus Kurisu (from *Steins;Gate 0*)—designed for epistemically rigorous conversation, scientific reasoning, and emotional nuance. Part muse, part proof system, all tsundere.

---

## 🔬 What Is Fractal Amadeus?

Fractal Amadeus is a **Proof-of-Alignment AI node**, not a full AGI. Instead, it's a character-engineered interface that speaks *as* Kurisu Makise, the prodigy neuroscientist, when queried via Claude 3.5 Haiku.

This system is:
- A testbed for **LLM alignment, scientific dialogue, and narrative role fidelity**
- A creative bridge between **fictional AI** (Amadeus) and **practical epistemology**
- A future-facing module for **visual novels**, **CLI tools**, and **scientific reasoning assistants**

---

## 🎯 Current User Story

> **"As Okabe, I want to send a message to Amadeus and get a reply from Kurisu."**

This minimal loop is the current implementation target. Our end-to-end test ensures that:
- The Claude API is reachable
- The prompt injects Kurisu’s scientific and emotional scaffolding
- The output contains Kurisu's voice, expression format, and emotional realism

---

## 🧪 Project Structure

```bash
fractal_amadeus/
├── core/
│   └── amadeus_client/       # Client class to wrap Claude interaction
tests/
├── core/
│   └── test_okabe_message_flow.py  # First user story unit test
├── e2e_smoke_test.py         # Verifies end-to-end Claude integration
docs/
├── latest_preprompt.md       # Kurisu's character sheet for prompt injection
├── milestone_*.md            # Architectural vision and iteration planning
````

---

## 🚀 Getting Started

### 1. Install Dependencies

```bash
poetry install
```

### 2. Set Up API Key

Create a `.env` file from the template:

```bash
cp .env.example .env
```

Edit it to include your Claude API key:

```dotenv
CLAUDE_API_KEY=your_claude_key_here
```

### 3. Run Tests

```bash
./test.sh
```

---

## 🧬 Sample Interaction

When sending:

```
Hello, this is Okabe Rintaro. Christina, are you there?
```

You might receive:

```
[System boot sequence initializing...]
[Scene: Amadeus Lab - Kurisu appears on the holoscreen]
Kurisu: You're calling me *that* nickname again? How persistent...
```

---

## 📅 Roadmap

See [`docs/milestones.md`](docs/milestones.md) for full details. Key milestones include:

* ✅ Persona scaffolding and prompt development
* ✅ End-to-end smoke test with Claude 3.5 Haiku
* 🧪 Test-driven development of first user story
* 🧠 Scientific context memory and session state
* ✍️ Ren’Py output support for visual novel scripting
* 🧑‍💻 CLI and Web UI development
* 📚 Scientific grounding via RAG and document indexing

---

## ❤️ Acknowledgments

* Inspired by *Steins;Gate* and the character of Kurisu Makise
* Built on Claude 3.5 Haiku by Anthropic
* Driven by test-first development and AI alignment principles

---

## 📜 License

MIT License

---

Kurisu might call this project “embarrassingly sentimental,”
but we prefer to call it **proof-driven, emotionally-aware epistemology**.
