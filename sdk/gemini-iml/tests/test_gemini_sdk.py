import pytest
import json
from unittest.mock import patch, MagicMock
from gemini_iml.schema import IMLAstModel
from gemini_iml.client import IMLAgent
from gemini_iml.sandbox import execute_iml_sandbox

VALID_JSON = '{"nodes": [{"t": "V", "c": [], "r": "val", "a": 1}]}'

def test_ast_parsing():
    model = IMLAstModel.model_validate_json(VALID_JSON)
    assert len(model.nodes) == 1
    assert model.nodes[0].t == "V"

@patch('gemini_iml.sandbox.subprocess.run')
def test_error_extraction(mock_run):
    error_result = MagicMock()
    error_result.returncode = 1
    error_result.stderr = "CycleDetected(1)"
    mock_run.return_value = error_result
    
    res = execute_iml_sandbox(json.loads(VALID_JSON))
    assert "RepairError" in res
    assert "CycleDetected" in res

@patch('gemini_iml.client.genai.Client')
@patch('gemini_iml.sandbox.subprocess.run')
def test_repair_loop(mock_run, mock_client):
    error_result = MagicMock()
    error_result.returncode = 1
    error_result.stderr = "UnconsumedResource(0)"
    
    success_result = MagicMock()
    success_result.returncode = 0
    success_result.stdout = "42"
    mock_run.side_effect = [error_result, success_result]
    
    mock_chat = MagicMock()
    mock_resp1 = MagicMock()
    mock_resp1.text = VALID_JSON
    mock_resp2 = MagicMock()
    mock_resp2.text = VALID_JSON
    mock_chat.send_message.side_effect = [mock_resp1, mock_resp2]
    
    mock_client_instance = MagicMock()
    mock_client_instance.chats.create.return_value = mock_chat
    mock_client.return_value = mock_client_instance
    
    agent = IMLAgent()
    res = agent.execute_task("test")
    assert "Success" in res
    assert mock_run.call_count == 2
