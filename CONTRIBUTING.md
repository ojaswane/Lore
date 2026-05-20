# Contributing to Lore

First off — thanks for being here. Lore is being built in the open and every contribution matters, whether it's a bug fix, a new feature, a typo, or just trying it out and telling us what broke.

---

## Before you start

Lore is pre-alpha. Things are incomplete, things will break, and the architecture is still evolving. That's not a warning to stay away — it's an invitation to help shape it.

If you're unsure whether your idea fits, open an issue first and talk about it. No PR should be a surprise.

---

## How to contribute

### 1. Find something to work on

- Check the [issues](../../issues) tab for open tasks
- Look for issues tagged `good first issue` if you're new
- Or open a new issue if you found a bug or have an idea

### 2. Fork and clone

```bash
git clone https://github.com/yourusername/lore.git
cd lore
pip install -r requirements.txt
```

### 3. Create a branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/what-you-are-fixing
```

### 4. Make your changes

Keep changes focused. One thing per PR. A PR that fixes a bug AND adds a feature AND refactors something is hard to review.

### 5. Test it

Run Lore and make sure it actually works before submitting:

```bash
python main.py
```

If you're touching the storage layer, make sure sessions save and load correctly. If you're touching the UI, make sure the terminal still feels like a terminal.

### 6. Open a pull request

Push your branch and open a PR against `main`. In the PR description, explain:

- what you changed
- why you changed it
- how to test it

---

## Good first issues to pick up

If you're new and want somewhere to start:

- improving compression logic in `core/compressor.py`
- adding keyboard shortcuts to the Textual UI
- writing tests for the SQLite storage layer
- improving ANSI escape code handling in `core/shell.py`
- improving the `lore search` output formatting
- writing better error messages when the shell crashes

---

## Code style

- Python 3.11+
- keep functions small and single-purpose
- comment anything that isn't obvious
- no external dependencies without a good reason — we keep the stack lean

---

## What we're not looking for (right now)

- cloud sync features
- electron or web-based UI
- anything that requires an account or internet connection

Lore is local first. Keep it that way.

---

## Questions?

Open an issue and tag it `question`. No question is too small.

---

*Close the terminal. The lore stays.*