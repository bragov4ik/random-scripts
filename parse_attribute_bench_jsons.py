import json
import os
import csv
from collections import OrderedDict,defaultdict


# Directory containing the JSON files
json_dir = '../sora2-network/benches/'

# List to store the extracted data
data = defaultdict(lambda: [])

# Loop through all JSON files in the directory
for filename in os.listdir(json_dir):
    if filename.endswith('.json'):
        with open(os.path.join(json_dir, filename), 'r') as json_file:
            json_data = json.load(json_file)
            
            for entry in json_data:
                row = OrderedDict()
                row['filename'] = filename
                extrinsic_name = entry['benchmark'].rsplit("_", 1)[0]
                time_results = entry['time_results'][0]['extrinsic_time']
                db_reads = entry['db_results'][0]['reads']
                db_writes = entry['db_results'][0]['writes']
                
                row["time"] = time_results
                row["reads"] = db_reads
                row["writes"] = db_writes
                data[extrinsic_name].append(row)

data = {name: sorted(rows, key=lambda row: row['filename'].split("_")) for name, rows in data.items()}

# Define the output CSV file
csv_file = 'output.csv'

# Write the data to the CSV file
with open(csv_file, 'w', newline='') as csvfile:
    fieldnames = data[list(data.keys())[0]][0]
    writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
    writer.writeheader()

    # print(data)
    for extrinsic, rows in data.items():
        extrinsic_name_row = {n: "" for n in fieldnames}
        # field_name_row = {n: n for n in fieldnames}
        extrinsic_name_row["filename"] = extrinsic
        writer.writerow(extrinsic_name_row)
        # writer.writerow(field_name_row)
        for row in rows:
            writer.writerow(row)

print(f'Data has been extracted and saved to {csv_file}.')
