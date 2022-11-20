import bisect

class BTreeNode:
    def __init__(self, leaf=False):
        self.leaf = leaf
        self.keys = []
        self.child = []

class BTree:
    def __init__(self, t=10, N=15):
        self.root = BTreeNode(True)
        self.refarr = [i for i in range(15)]
        self.t = t

    def init_refarr(self, arr):
        self.refarr = arr

    def insert_node(self, val, node = None, MAX_NODE=2):
        if not node:
            node = self.root
        i = bisect.bisect_right(node.keys, val)
        # if it's full, try to insert in child of the node
        if len(node.keys) == MAX_NODE:
            if node.leaf:
                midKey = node.keys[int(len(node.keys)/2)]
                # TODO: fix to O(1)
                node.keys.insert(i, val)
                node.keys.remove(midKey)
                # print(node.keys)
                # TODO: perhaps should divide child also like when creating new root
                return midKey
            else:
                retKey = self.insert_node(val, node.child[i])
                midKey = node.keys[int(len(node.keys)/2)]
                i = bisect.bisect_right(node.keys, retKey)
                node.keys.insert(i, retKey)
                node.keys.remove(midKey)
                # print(node.keys)
                # if it's root, create new root.
                if node == self.root:
                    newRoot = BTreeNode()
                    newRoot.keys = [midKey]
                    self.root = newRoot
                    leftNode = BTreeNode()
                    rightNode = BTreeNode()
                    leftNode.keys.extend(node.keys[:int(len(node.keys)/2)])
                    rightNode.keys.extend(node.keys[int(len(node.keys)/2):])
                    leftNode.child.extend(node.child[:int(len(node.child)/2)])
                    rightNode.child.extend(node.child[int(len(node.child)/2):])
                    newRoot.child = [leftNode, rightNode]
                else:
                    return midKey
        else:
            node.keys.insert(i, val)


    def dump_tree(self):
        que = [[self.root]]
        while que:
            nodes = que.pop(0)
            print([node.keys for node in nodes])

            for node in nodes:
                nodes = []
                for childNode in node.child:
                    nodes.append(childNode)
                if nodes:
                    que.append(nodes)

    def initialize(self, L=0, R=15, D=3):
        if R-L > D:
            node = BTreeNode(False)
            n = int((R-L)/D)
            for i in range(1, D):
                node.keys.append(L+n*i)
                enode = self.initialize(L=L+n*(i-1), R=L+n*i)
                node.child.append(enode)
            enode = self.initialize(L=L+n*(D-1), R=R)
            node.child.append(enode)
        else:
            node = BTreeNode(True)
            # print(L, R)
            for i in range(L+1, R):
                node.keys.append(i)
        # print([self.refarr[key] for key in node.keys], node.child)
        return node

def main():
    btree = BTree()
    btree.init_refarr([1, 2, 4, 6, 8, 10, 12, 13, 15, 16, 17, 18, 20, 22, 25])
    btree.root = btree.initialize()
    # btree.dump_tree()
    btree.insert_node(15)
    btree.dump_tree()

if __name__ == "__main__":
    main()
