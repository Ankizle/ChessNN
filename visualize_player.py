import networkx as nx
import matplotlib
import matplotlib.pyplot as plt
import json

matplotlib.use("GTK3Agg")

class Graph:

    def __init__(self):
        self.visual = []

    def add_edge(self, a, b):
        self.visual.append([a, b])

    def display(self):
        G = nx.DiGraph()
        G.add_edges_from(self.visual)
        nx.draw_networkx(G)
        plt.show()

filename = input("Filename: ")

gv = Graph()

with open(filename) as f:
    data = json.load(f)["network"]

    for (k, v) in enumerate(data):
        for i in v:
            gv.add_edge(k, i[0])

    gv.display()