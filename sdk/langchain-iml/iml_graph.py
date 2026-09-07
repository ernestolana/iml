import json
from typing import TypedDict, List, Dict, Any, Optional
from langgraph.graph import StateGraph, END
from langchain_core.messages import BaseMessage, HumanMessage, AIMessage
from langchain_core.prompts import ChatPromptTemplate
from langchain_core.runnables import RunnableConfig

from .iml_parser import IMLOutputParser
from .iml_tool import execute_iml_sandbox

class IMLState(TypedDict):
    task: str
    messages: List[BaseMessage]
    current_ast: Optional[List[Dict[str, Any]]]
    execution_result: Optional[str]
    errors: int

def create_iml_repair_loop(llm, max_retries: int = 3):
    """Creates a LangGraph sub-graph template for the IML repair loop."""
    
    parser = IMLOutputParser()
    
    prompt = ChatPromptTemplate.from_messages([
        ("system", "You are an AI agent generating IML. Output the AST as a JSON array. DO NOT output anything else except the JSON array."),
        ("human", "{task}"),
        ("placeholder", "{messages}")
    ])
    
    chain = prompt | llm | parser
    
    def generate_ast(state: IMLState, config: RunnableConfig):
        try:
            ast = chain.invoke(state, config=config)
            return {"current_ast": ast}
        except Exception as e:
            return {
                "execution_result": f"ParseError: {str(e)}",
                "errors": state.get("errors", 0) + 1
            }
            
    def execute_ast(state: IMLState):
        if not state.get("current_ast"):
            return {}
        
        result = execute_iml_sandbox.invoke({"ast": state["current_ast"]})
        
        # Keep track of the repair loop conversation
        new_messages = state.get("messages", []) + [
            AIMessage(content=json.dumps(state["current_ast"])),
            HumanMessage(content=result)
        ]
        
        return {
            "execution_result": result,
            "messages": new_messages,
            "errors": state.get("errors", 0) + (1 if "RepairError" in result else 0)
        }
        
    def should_continue(state: IMLState) -> str:
        result = state.get("execution_result", "")
        if "Success" in result:
            return END
        
        errors = state.get("errors", 0)
        if errors >= max_retries:
            return END
            
        return "generate"

    workflow = StateGraph(IMLState)
    
    workflow.add_node("generate", generate_ast)
    workflow.add_node("execute", execute_ast)
    
    workflow.set_entry_point("generate")
    workflow.add_edge("generate", "execute")
    
    workflow.add_conditional_edges(
        "execute",
        should_continue,
        {
            "generate": "generate",
            END: END
        }
    )
    
    return workflow.compile()
