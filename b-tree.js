const children_max = 3;
const key_max = children_max - 1;

class Node {
  constructor() {
    // key & value infact
    this.keys = [];
    this.children = [];
    this.parent = null;
  }
}

function getPos(keys, key) {
  let pos = -1;
  for (let i = 0; i < keys.length; i++) {
    if (key < keys[i]) {
      pos = i;
      break;
    } else if (key > keys[i] && i === keys.length - 1) {
      pos = i + 1;
      break;
    } else if (key > key[i] && key < key[i + 1]) {
      pos = i + 1;
      break;
    }
  }

  if (pos === -1) {
    throw new Error(`can find pos for ${key} in node: ${node.keys}`);
  }

  return pos;
}

class Tree {
  constructor() {
    this._root = new Node();
  }

  _searchNearest(key) {
    return (function _search(node) {
      const keyMaxNow = node.keys.length - 1;
      for (let i = 0; i <= keyMaxNow; i++) {
        if (i === 0 && key < node.keys[i] && node.children[i]) {
          // left most
          return _search(node.children[i]);
        } else if (
          i === keyMaxNow &&
          key > node.keys[i] &&
          node.children[i + 1]
        ) {
          // right most
          return _search(node.children[i + 1]);
        } else if (
          key > node.keys[i] &&
          key < node.keys[i + 1] &&
          node.children[i + 1]
        ) {
          return _search(node.children[i + 1]);
        }
      }

      return node;
    })(this._root);
  }

  insert(key, node = null) {
    if (node === null) {
      node = this._searchNearest(key);
    }

    if (node.keys.length === 0) {
      // root init
      node.keys.push(key);
    } else if (node.keys.length < key_max) {
      const pos = getPos(node.keys, key);
      node.keys.splice(pos, 0, key);
    } else {
      // split node
      const toSplitKeys = node.keys.slice(0);
      const pos = getPos(toSplitKeys, key);
      toSplitKeys.splice(pos, 0, key);
      // key_max is an even
      const medianIdx = key_max / 2;
      const medianKey = toSplitKeys[medianIdx];
      // split keys
      const leftNode = new Node();
      leftNode.keys = toSplitKeys.slice(0, medianIdx);
      const rightNode = new Node();
      rightNode.keys = toSplitKeys.slice(medianIdx + 1, toSplitKeys.length);
      // split children
      if (node.children.length > 0) {
        leftNode.children = node.children.slice(0, medianIdx + 1);
        rightNode.children = node.children.slice(
          medianIdx + 1,
          node.children.length
        );
        leftNode.children.forEach((c) => (c.parent = leftNode));
        rightNode.children.forEach((c) => (c.parent = rightNode));
      }

      if (node.parent) {
        const oldChildIdx = node.parent.children.indexOf(node);
        leftNode.parent = node.parent;
        node.parent.children[oldChildIdx] = leftNode;
        rightNode.parent = node.parent;
        node.parent.children.splice(oldChildIdx + 1, 0, rightNode);
        // recursive insert
        this.insert(medianKey, node.parent);
      } else {
        // new root
        const newRoot = new Node();
        newRoot.keys = [medianKey];
        leftNode.parent = newRoot;
        rightNode.parent = newRoot;
        newRoot.children = [leftNode, rightNode];
        this._root = newRoot;
      }
    }
  }

  printTree() {
    const queue = [{ node: this._root, height: 0 }];
    let highest = 0;
    let output = "";
    while (queue.length > 0) {
      const { node, height } = queue.shift();
      if (height > highest) {
        output += "\n";
        highest = height;
      }
      output += `${JSON.stringify(node.keys)}\t`;
      for (const c of node.children) {
        queue.push({ node: c, height: height + 1 });
      }
    }
    console.log(output);
  }
}

const tree = new Tree();
tree.insert(1);
tree.insert(2);
tree.insert(5);
tree.insert(6);
tree.printTree();
console.log("\n");
tree.insert(7);
tree.insert(3);
tree.insert(4);
tree.printTree();
