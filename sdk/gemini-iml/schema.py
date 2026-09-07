from pydantic import BaseModel, Field
from typing import List, Optional, Any, Dict

class IMLNode(BaseModel):
    t: Dict[str, Any] = Field(description="Type definitions")
    c: List[int] = Field(description="Children indices")
    r: Optional[str] = Field(None, description="Rationale for this node")
    o: Optional[str] = Field(None, description="Operation to perform")
    f: Optional[int] = Field(None, description="Fuel limit for sandbox execution")
    a: Optional[Any] = Field(None, description="Arguments or literal values")

class IMLAstModel(BaseModel):
    nodes: List[IMLNode] = Field(description="A flat JSON array of nodes representing a Directed Acyclic Graph (DAG) AST")
