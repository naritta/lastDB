class BTreeNode:
  def __init__(self, leaf=False):
    self.leaf = leaf
    self.keys = []
    self.child = []

class BTree:
  def __init__(self, t):
    self.root = BTreeNode(True)
    self.t = t

  def search_key(self, k, x=None):
    if x:
      i = 0
      while i < len(x.keys) and k > x.keys[i][0]:
        i += 1
      if i < len(x.keys) and k == x.keys[i][0]:
        return (x, i)
      elif x.leaf:
        return None
      else:
        return self.search_key(k, x.child[i])
    else:
      return self.search_key(k, self.root)

def main():
    btree = BTreeNode(False)
    print(btree)

if __name__ == "__main__":
    main()
