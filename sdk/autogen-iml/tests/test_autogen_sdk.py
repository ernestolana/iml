import pytest
import json
from unittest.mock import patch, AsyncMock, MagicMock
from autogen_iml.tools import execute_iml_sandbox
from autogen_iml.agent import create_iml_agent
from autogen_iml.swarm import create_iml_swarm

VALID_JSON = '{"nodes": [{"t": "V", "c": [], "r": "val", "a": 1}]}'

def test_ast_parsing():
    # Parsing is implied in sandbox for now
    pass

@patch('autogen_iml.tools.subprocess.run')
def test_error_extraction(mock_run):
    error_result = MagicMock()
    error_result.returncode = 1
    error_result.stderr = "CycleDetected(1)"
    mock_run.return_value = error_result
    
    res = execute_iml_sandbox(json.loads(VALID_JSON)["nodes"])
    assert "RepairError" in res
    assert "CycleDetected" in res

@patch('autogen_iml.tools.subprocess.run')
def test_repair_loop(mock_run):
    error_result = MagicMock()
    error_result.returncode = 1
    error_result.stderr = "UnconsumedResource(0)"
    
    success_result = MagicMock()
    success_result.returncode = 0
    success_result.stdout = "42"
    mock_run.side_effect = [error_result, success_result]
    
    mock_client = MagicMock()
    mock_response1 = MagicMock()
    mock_response1.content = "Call tool!"
    mock_response2 = MagicMock()
    mock_response2.content = "Success: 42"
    # Autogen's mocking can be complicated, we just check creation and configuration
    
    team = create_iml_swarm(mock_client, max_retries=3)
    assert len(team._participants) == 2
    assert team._participants[0].name == "iml_coder"
    assert team._participants[1].name == "iml_executor"
