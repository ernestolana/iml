import pytest
import json
from unittest.mock import patch, MagicMock
from langchain_iml.iml_parser import IMLOutputParser
from langchain_iml.iml_tool import execute_iml_sandbox
from langchain_iml.iml_graph import create_iml_repair_loop
from langchain_core.messages import AIMessage, HumanMessage

VALID_JSON = '{"nodes": [{"t": "V", "c": [], "r": "val", "a": 1}]}'

def test_ast_parsing():
    parser = IMLOutputParser()
    res = parser.parse(VALID_JSON)
    assert "nodes" in res
    assert len(res["nodes"]) == 1
    assert res["nodes"][0]["t"] == "V"

@patch('langchain_iml.iml_tool.subprocess.run')
def test_error_extraction(mock_run):
    error_result = MagicMock()
    error_result.returncode = 1
    error_result.stderr = "CycleDetected(1)"
    mock_run.return_value = error_result
    
    res = execute_iml_sandbox.invoke({"ast": json.loads(VALID_JSON)})
    assert "RepairError" in res
    assert "CycleDetected" in res

@patch('langchain_iml.iml_tool.subprocess.run')
def test_repair_loop(mock_run):
    error_result = MagicMock()
    error_result.returncode = 1
    error_result.stderr = "UnconsumedResource(0)"
    
    success_result = MagicMock()
    success_result.returncode = 0
    success_result.stdout = "42"
    mock_run.side_effect = [error_result, success_result]
    
    from langchain_core.language_models import FakeListChatModel
    mock_llm = FakeListChatModel(responses=[VALID_JSON, VALID_JSON])
    
    graph = create_iml_repair_loop(mock_llm, max_retries=3)
    
    res = graph.invoke({
        "task": "test",
        "errors": 0,
        "messages": []
    })
    
    assert "execution_result" in res
    assert "Success" in res["execution_result"]
    assert mock_run.call_count == 2
