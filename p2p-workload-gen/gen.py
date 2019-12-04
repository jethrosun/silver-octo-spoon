import json

def generate_output(num):
    current_data = {}
    current_data['torrents_files'] = data['torrents_files'][:num]

    with open(str(num) + '_workload.json', 'w') as outfile:
        json.dump(current_data, outfile)

data = {}
data['torrents_files'] = []

filepath = 'torrent-list.txt'
with open(filepath) as fp:
    line = fp.readline()
    while line:
        data['torrents_files'].append(str(line.strip()))
        line = fp.readline()


print(len(data))
print(data)

for i in [1, 10, 20, 40, 50, 75,100,150,200]:
    generate_output(i)

