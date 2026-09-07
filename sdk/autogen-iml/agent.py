from autogen_agentchat.agents import AssistantAgent
from .tools import execute_iml_sandbox_tool

def create_iml_agent(model_client, name="iml_agent"):
    system_message = (
        "You are an AI agent generating IML. Output the AST as a JSON array adhering to IML JSON Schema Draft 2020-12. "
        "Your schema requires keys: 't', 'c', 'r', 'o', 'f', 'a'. Use the execute_iml_sandbox tool to test your AST. "
        "If you encounter a RepairError (like UnconsumedResource or CycleDetected), correct your AST and try again."
    )
    
    return AssistantAgent(
        name=name,
        model_client=model_client,
        system_message=system_message,
        tools=[execute_iml_sandbox_tool]
    )
