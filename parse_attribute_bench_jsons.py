import json
import os
import csv

# Directory containing the JSON files
json_dir = '../sora2-network/benches/'

# List to store the extracted data
data = []

# Loop through all JSON files in the directory
for filename in os.listdir(json_dir):
    if filename.endswith('.json'):
        with open(os.path.join(json_dir, filename), 'r') as json_file:
            json_data = json.load(json_file)
            
            row = {'Filename': filename}
            for entry in json_data:
                extrinsic_name = entry['benchmark'].rsplit("_", 1)[0]
                time_results = entry['time_results'][0]['extrinsic_time']
                db_reads = entry['db_results'][0]['reads']
                db_writes = entry['db_results'][0]['writes']
                
                row[extrinsic_name+"_time"] = time_results
                row[extrinsic_name+"_reads"] = db_reads
                row[extrinsic_name+"_writes"] = db_writes
                

            data.append(row)

data.sort(key=lambda row: row['Filename'])

# Define the output CSV file
csv_file = 'output.csv'

# Write the data to the CSV file
with open(csv_file, 'w', newline='') as csvfile:
    fieldnames = data[0].keys()
    writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
    
    writer.writeheader()
    for row in data:
        writer.writerow(row)

print(f'Data has been extracted and saved to {csv_file}.')
