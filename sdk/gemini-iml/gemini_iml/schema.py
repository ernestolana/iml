from pydantic import BaseModel, Field
from typing import List, Optional, Any, Dict

class IMLNode(BaseModel):
    t: str = Field(description="Node Type / Opcode (e.g., L, D, V, T, C, B, P)")
    c: List[int] = Field(description="Children / Input dependency indices")
    r: Optional[str] = Field(None, description="Semantic Rationale")
    o: Optional[str] = Field(None, description="Output / Resource Handle Identifier")
    a: Optional[Any] = Field(None, description="Inline Arguments / Literals")

class IMLAstModel(BaseModel):
    nodes: List[IMLNode] = Field(description="A flat JSON array of nodes representing a Directed Acyclic Graph (DAG) AST")
