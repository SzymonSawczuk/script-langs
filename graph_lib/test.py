import sys
from  graph_lib import EdgeInt, EdgeString, GraphInt, GraphString, NodeInt, NodeString

def test_string():
    graph_string = GraphString()
    graph_string.add_vertex("Poland")
    graph_string.add_vertex("Deutschland")
    graph_string.add_vertex("France")
    graph_string.add_vertex("UK")
    graph_string.add_vertex("Sweden")

    graph_string.add_edge("Poland", "Deutschland", 400)
    graph_string.add_edge("Deutschland", "France", 300)
    graph_string.add_edge("France", "UK", 500)
    graph_string.add_edge("Poland", "Sweden", 600)

    graph_string.show_graph()
    print()
    graph_string.dfs_algorithm()
    print()
    graph_string.bfs_algorithm("Poland")

def test_int():
    node_12 = NodeInt(12)
    node_12.set_value(11)

    graph_int = GraphInt()
    graph_int.add_vertex(node_12.get_value())
    graph_int.add_vertex(12)
    graph_int.add_vertex(3)
    graph_int.add_vertex(8)
    graph_int.add_vertex(10)
    graph_int.add_vertex(123)

    edge_1 = EdgeInt(4000, 12, 3)
    graph_int.add_edge(edge_1.get_first_node(), edge_1.get_second_node(), edge_1.get_weight())
    graph_int.add_edge(3, 8, 3009, True)
    graph_int.add_edge(8, 10, 5020)
    graph_int.add_edge(12, 123, 0, True)

    graph_int.show_graph()
    print()
    graph_int.dfs_algorithm()
    print()
    graph_int.bfs_algorithm(8)


def add_vertex(graph):
    name = input('Enter name of vertex: ') 
    graph.add_vertex(name)

def add_edge(graph):
    first_node = input('Enter name of first vertex: ') 
    second_node = input('Enter name of second vertex: ') 
    weight = int(input('Enter weight: ')) 
    is_directed = bool(input('Is directed? (0 or 1): '))
    graph.add_edge(first_node, second_node, weight, is_directed)

def remove_vertex(graph):
    name = input('Enter name of vertex: ') 
    graph.remove_vertex(name)

def remove_edge(graph):
    first_node = input('Enter name of first vertex: ') 
    second_node = input('Enter name of second vertex: ') 
    is_directed = bool(input('Is directed? (0 or 1): '))
    graph.remove_edge(first_node, second_node, is_directed)

def BFS(graph):
    start = input('Enter name of starting vertex: ') 
    graph.bfs_algorithm(start)

def DFS(graph):
    graph.dfs_algorithm()

def show_graph(graph):
    graph.show_graph()

def test_interactive():
    graph = GraphString()

    while(True):
        print("Menu\n1.Add vertex\n2.Add Edge\n3.remove_vertex\n4.remove_edge\n5.BFS\n6.DFS\n7.Show\n8.Quit")
        option = int(input('Enter your choice: '))  

        if option == 1:
            add_vertex(graph)
        elif option == 2:
            add_edge(graph)   
        elif option == 3:
            remove_vertex(graph)  
        elif option == 4:
            remove_edge(graph)     
        elif option == 5:
            BFS(graph)
        elif option == 6:
            DFS(graph)
        elif option == 7:
            show_graph(graph)
        elif option == 8:
            sys.exit()
        else:
            print("Wrong option")                

     


if __name__ == "__main__":
    # test_string()
    # print()
    # test_int()
    test_interactive()

    