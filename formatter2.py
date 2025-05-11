import json
import os
dir_path = '/home/wiwi/rust-projects/blackjack/models'
for file_name in os.listdir(dir_path):
    if file_name.endswith('.json'):
        file_path = os.path.join(dir_path, file_name)
        # Charger le JSON
        with open(file_path, 'r') as file:
            data = json.load(file)
        new_data = []
        for i, row in enumerate(data):
            new_data.append([])
            for j in row:
                for pp in j.keys():
                    new_data[i].append(str(pp))

        # Réécrire le JSON avec une liste par ligne
        with open(file_path, 'w') as f:
            f.write(json.dumps(new_data))