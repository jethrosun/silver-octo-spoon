import json

data = {}
data['torrents_files'] = []

filepath = 'torrent-list.txt'
with open(filepath) as fp:
    line = fp.readline()
    print(line)
    while line:
        data['torrents_files'].append(
            str(line.strip())
        )
        line = fp.readline()


with open('p2p-workload.json', 'w') as outfile:
    json.dump(data, outfile)
