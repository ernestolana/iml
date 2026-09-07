import json
from typing import Any, Dict, List
from langchain_core.exceptions import OutputParserException
from langchain_core.output_parsers import BaseOutputParser

class IMLOutputParser(BaseOutputParser[Dict[str, Any]]):
    """Parses LLM output into the ultra-terse IML JSON Schema."""
    
    def parse(self, text: str) -> Dict[str, Any]:
        try:
            # Simple extraction if LLM outputs markdown block
            text = text.strip()
            if text.startswith("```json"):
                text = text[7:]
            if text.startswith("```"):
                text = text[3:]
            if text.endswith("```"):
                text = text[:-3]
            
            parsed = json.loads(text.strip())
            if not isinstance(parsed, dict) or "nodes" not in parsed:
                raise ValueError("IML AST must be a JSON object with a 'nodes' array")
                
            return parsed
        except json.JSONDecodeError as e:
            raise OutputParserException(f"Failed to parse IML JSON: {e}")
        except Exception as e:
            raise OutputParserException(f"IML validation error: {e}")

    @property
    def _type(self) -> str:
        return "iml_output_parser"
