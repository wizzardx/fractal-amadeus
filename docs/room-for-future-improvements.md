
Fractal Amadeus Project Review
https://chatgpt.com/c/6828d41c-1d1c-8002-bf64-d5baa0444e71

---

### 📌 Room for Improvement / Suggestions

1. **Memory Persistence & State**:
   You've mentioned rotating memory of the past 100 messages. That’s a good start, but:

   * Consider a local vector store (e.g., FAISS or Chroma) to track scientific dialogue over time for long-term memory.
   * Use fingerprinting to detect similar questions previously answered and optionally summarize earlier threads in Claude input context.

2. **Scientific Accuracy Checks**:

   * You might add a `tests/accuracy/` suite using JSON-based problem/answer pairs for Kurisu to evaluate, and then autograde correctness with metadata tags like confidence, references used, etc.

3. **Ren’Py Output Support**:

   * I assume future output from Kurisu is meant to be rendered directly into Ren’Py VN format.
   * Consider building a small tool or script to extract Ren’Py-ready segments from full Claude responses.

4. **CLI Prototype**:

   * You allude to a minimal CLI—great idea. A lightweight TUI (e.g., with `rich` or `textual`) where Kurisu speaks back could reinforce the “scientific VN lab partner” vibe.

5. **Web Interface**:

   * When ready, a retro-futuristic UI that mimics the Amadeus terminal in Steins;Gate could become a powerful engagement piece. Even just a static chatlog visualizer would work.

