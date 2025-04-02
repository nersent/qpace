# qPACE: Pine compiler

### Prerequisites

Please, ensure you have [Node.js](https://nodejs.org/) 18 or higher installed. You can check your Node.js version with:

```bash
node -v
```

You may also need [Python](https://www.python.org/) 3.10 or higher installed. You can check your Python version with:

```bash
python3 --version
```

### Running

1. Install

CLI:

```bash
npm install qpace --global
```

Python:

```bash
pip3 install qpace
```

2. Authenticate

qPACE account is required to use the compiler. You can create it [here](https://qpace.dev/auth).

```bash
qpace login
```

3. Create a new QPC project

```bash
cd /path/to/your/project

qpc init
```

4. Check (optional)

This step is optional, but it is recommended before building. It performs fast semantic, syntax, and type checks on your Pine code.

```bash
qpc check
```

5. Build

Builds a fully usable Python wheel and installs it automatically.

```bash
qpc build
```

or

```bash
qpc build --target python
```

6. Import

To use it from Python, you need to import module, which name is located in your `.qpace.json` config file `python -> package`.

For examle:

`.qpace.json`

```json
{
  "python": {
    "package": "<PACKAGE_NAME>"
  }
}
```

`my_library.pine`

```pine
export add(float x, float y) =>
    x + y
```

`main.py`

```python
# Make sure you use correct name!
import <PACKAGE_NAME> as pine

pine.my_library.add(5.0, 25.0)
```

6. Run

```bash
python main.py
```
