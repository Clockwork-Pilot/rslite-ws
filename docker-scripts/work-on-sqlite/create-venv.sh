set -euo pipefail

python3 -m venv /unsafe_rust_fixer/.venv &&
source /unsafe_rust_fixer/.venv/bin/activate &&
pip install -r /unsafe_rust_fixer/requirements.txt &&
pip install -r /plugin/knowledge_tool/requirements.txt &&
pip install -r /plugin/requirements.txt