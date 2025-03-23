# qPACE: Pine compiler

### Prerequisites

Please, ensure you have [Node.js](https://nodejs.org/) 18 or higher installed. You can check your Node.js version with:

```bash
node -v
```

### Running

1. Install CLI

```bash
npm install qpace --global
```

2. Authenticate

qPACE account is required to use the compiler. You can create it [here](https://qpace.dev/auth).

```bash
qpace login
```

3. Create new QPC project

```bash
cd /path/to/your/project

qpc init
```

4. Check (optional)

This step is optional, but it is recommended before building. It performs semantic, syntax, and type checks on your Pine code.

```bash
qpc check
```

5. Build

Builds a fully usable Python wheel and installs it automatically.

```bash
qpc build --target python
```

To use it from python, you need to import module, which name is located in your `.qpace.json` config file `python -> package`.

For examle:

`.qpace.json`

```json
{
  "python": {
    "package": "python_pine_example"
  }
}
```

`my_library.pine`

```pine
export add(float x, float y) =>
    x + y
```

```python
import python_pine_example as pine

pine.my_library.add(5.0, 25.0)
```

6. Run

```bash
python main.py
```
