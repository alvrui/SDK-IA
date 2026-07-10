from datetime import datetime
from typing import Dict, List, Optional

class Agent:
    def __init__(self, name: str, agent_id: str):
        self.name = name
        self.agent_id = agent_id
        self.last_used = datetime.now()

class AgentManager:
    def __init__(self):
        self.agents: Dict[str, Agent] = {}

    def add_agent(self, name: str, agent_id: str) -> None:
        self.agents[agent_id] = Agent(name=name, agent_id=agent_id)

    def get_agents_sorted(self) -> List[Agent]:
        return sorted(
            self.agents.values(),
            key=lambda x: x.last_used,
            reverse=True
        )

    def get_agent_by_id(self, agent_id: str) -> Optional[Agent]:
        return self.agents.get(agent_id)
