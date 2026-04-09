set -e

# called from user-entrypoint.sh script
python3 -m venv /venv &&
source /venv/bin/activate &&
pip install -r /plugin/knowledge_tool/requirements.txt &&
pip install -r /plugin/requirements.txt