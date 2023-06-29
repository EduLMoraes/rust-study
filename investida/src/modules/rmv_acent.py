from unidecode import unidecode
import csv

def remover_acentos(texto):
    return unidecode(texto)

input_file_path = './src/data/fills.csv'
output_file_path = './src/data/new_fills.csv'

with open(input_file_path, 'r', encoding='latin-1') as input_file, \
     open(output_file_path, 'w', newline='', encoding='utf-8') as output_file:
    
    reader = csv.reader(input_file)
    writer = csv.writer(output_file)
    
    for row in reader:
        processed_row = [remover_acentos(value) for value in row]
        writer.writerow(processed_row)
