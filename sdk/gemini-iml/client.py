import json
from google import genai
from google.genai import types
from .schema import IMLAstModel
from .sandbox import execute_iml_sandbox

class IMLAgent:
    def __init__(self, model_name: str = "gemini-2.5-pro"):
        self.client = genai.Client()
        self.model_name = model_name
        self.chat = self.client.chats.create(
            model=model_name,
            config=types.GenerateContentConfig(
                tools=[execute_iml_sandbox],
                response_schema=IMLAstModel,
                response_mime_type="application/json",
                temperature=0.1
            )
        )

    def execute_task(self, task: str, max_retries: int = 3) -> str:
        prompt = task
        
        attempts = 0
        while attempts < max_retries:
            response = self.chat.send_message(prompt)
            attempts += 1
            
            try:
                if not response.text:
                    prompt = "No text returned. Please output the JSON AST."
                    continue
                    
                ast_dict = json.loads(response.text)
                
                result = execute_iml_sandbox(ast_dict)
                
                if "Success" in result:
                    return result
                else:
                    # Trap or UnconsumedResource
                    prompt = f"RepairError trace:\n{result}\n\nPlease repair the AST and return the corrected JSON."
                    
            except json.JSONDecodeError as e:
                prompt = f"Failed to parse JSON response: {e}. Output must be valid JSON."
            except Exception as e:
                prompt = f"Unexpected error: {e}. Please try again."
                
        return f"Failed after {max_retries} attempts. Last result: {prompt}"
