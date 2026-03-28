set -euo pipefail

python3 -m venv /ra_ap_shell/.venv
source /ra_ap_shell/.venv/bin/activate
pip install -r /ra_ap_shell/requirements.txt
pip install -r /plugin/knowledge_tool/requirements.txt
pip install -r /plugin/requirements.txt