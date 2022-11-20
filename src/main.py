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

    # Insert node
    def insert(self, k):
        root = self.root
        if len(root.keys) == (2 * self.t) - 1:
            temp = BTreeNode()
            self.root = temp
            temp.child.insert(0, root)
            self.split_child(temp, 0)
            self.insert_non_full(temp, k)
        else:
            self.insert_non_full(root, k)

    # Insert nonfull
    def insert_non_full(self, x, k):
        i = len(x.keys) - 1
        if x.leaf:
            x.keys.append((None, None))
            while i >= 0 and k[0] < x.keys[i][0]:
                x.keys[i + 1] = x.keys[i]
                i -= 1
            x.keys[i + 1] = k
        else:
            while i >= 0 and k[0] < x.keys[i][0]:
                i -= 1
            i += 1
            if len(x.child[i].keys) == (2 * self.t) - 1:
                self.split_child(x, i)
                if k[0] > x.keys[i][0]:
                    i += 1
            self.insert_non_full(x.child[i], k)

    # Split the child
    def split_child(self, x, i):
        t = self.t
        y = x.child[i]
        z = BTreeNode(y.leaf)
        x.child.insert(i + 1, z)
        x.keys.insert(i, y.keys[t - 1])
        z.keys = y.keys[t: (2 * t) - 1]
        y.keys = y.keys[0: t - 1]
        if not y.leaf:
            z.child = y.child[t: 2 * t]
            y.child = y.child[0: t - 1]

    def dump_tree(self):
        que = [self.root]
        while que:
            node = que.pop(0)
            print(node.keys)
            for childNode in node.child:
                que.append(childNode)

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
    btree.dump_tree()

if __name__ == "__main__":
    main()
