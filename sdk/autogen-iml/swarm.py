from autogen_agentchat.conditions import TextMentionTermination, MaxMessageTermination
from autogen_agentchat.teams import RoundRobinGroupChat
from autogen_agentchat.agents import UserProxyAgent
from .agent import create_iml_agent

def create_iml_swarm(model_client, max_retries=5):
    assistant = create_iml_agent(model_client, name="iml_coder")
    
    # Runtime execution agent
    executor = UserProxyAgent(
        name="iml_executor",
        description="Executes tools on behalf of the IML coder.",
    )
    
    # Termination condition: halt if Success is in the output, or continue if RepairError.
    termination = TextMentionTermination("Success:") | MaxMessageTermination(max_retries * 2)
    
    team = RoundRobinGroupChat(
        participants=[assistant, executor],
        termination_condition=termination
    )
    
    return team
