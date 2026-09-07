import json
import subprocess
import tempfile
import os
from langchain_core.tools import tool
from typing import List, Dict, Any

@tool
def execute_iml_sandbox(ast: Dict[str, Any]) -> str:
    """Executes the given IML AST in the local wasm sandbox or returns a RepairError trace."""
    
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
        json.dump(ast, f)
        temp_path = f.name
        
    try:
        result = subprocess.run(
            ["iml", "run", "--sandbox", temp_path],
            capture_output=True,
            text=True,
            check=False
        )
        
        if result.returncode == 0:
            return f"Success:\n{result.stdout}"
        else:
            return f"RepairError:\n{result.stderr}"
    except FileNotFoundError:
        return "RepairError: local iml binary not found on PATH."
    except Exception as e:
        return f"RepairError: {str(e)}"
    finally:
        if os.path.exists(temp_path):
            os.remove(temp_path)
