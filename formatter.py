import json
import os

dir_path = '/home/wiwi/rust-projects/blackjack/models'
for file_name in os.listdir(dir_path):
    if file_name.endswith('.json'):
        file_path = os.path.join(dir_path, file_name)
        # Charger le JSON
        with open(file_path, 'r') as file:
            data = json.load(file)

        # Réécrire le JSON avec une liste par ligne
        with open(file_path, 'w') as f:
            formatted_json = json.dumps(data,)
            formatted_json = formatted_json.replace("],", "],\n")
            
            f.write(formatted_json)