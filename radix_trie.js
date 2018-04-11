// https://en.wikipedia.org/wiki/Radix_tree
// 如果自己要实现简单的router（不带参数，不带正则），这个数据结构就很厉害了

function printTree(root) {
  let queue = [];
  queue.push({ n: root, level: 0 });

  let deepest = 0;
  let str = "";
  while (queue.length !== 0) {
    let { n, level } = queue.shift();
    if (level > deepest) {
      str += "\n";
      deepest = level;
    }
    str += n.val + "    ";

    if (n.children) {
      for (let i = 0; i < n.children.length; i++) {
        queue.push({ n: n.children[i], level: level + 1 });
      }
    }
  }

  console.log(str);
}

function samePrefixN(str1, str2) {
  let shorter = str1.length < str2.length ? str1.length : str2.length;
  let N = 0;

  for (let i = 0; i < shorter; i++) {
    if (str1[i] === str2[i]) {
      N++;
    } else {
      break;
    }
  }

  return N;
}

function splitFromLast(word, letter) {
  let pivot = word.lastIndexOf(letter) + 1;
  let former = word.substring(0, pivot);
  let latter = word.substring(pivot, word.length);

  return { former, latter };
}

class RadixTrie {
  constructor() {
    this.root = { val: "ROOT", children: [] };
  }
  insert(word) {
    let lastNode = this.root;
    let cont = true;
    let remain = word;

    // try best to find
    while (cont) {
      let n = 0;
      for (let child of lastNode.children) {
        n = samePrefixN(remain, child.val);
        // same prefix
        if (n > 0) {
          lastNode = child;
          remain = remain.substring(n, remain.length);
          break;
        }
      }

      if (n === 0) {
        // not found
        cont = false;
      } else if (lastNode.children.length === 0) {
        // leaf node
        cont = false;
      } else if (lastNode.val.length !== n) {
        // just partial match
        cont = false;
      } else if (!remain) {
        // full match
        cont = false;
      } else {
        cont = true;
      }
    }

    // have to insert new node
    if (remain) {
      if (remain == word) {
        //insert to root
        lastNode.children.push({
          val: word,
          children: []
        });
      } else if (lastNode.children.length === 0) {
        // leaf
        if (word[word.length - 1] === lastNode.val[lastNode.val.length - 1]) {
          // lastNode become prefix
          lastNode.children.push({ val: "", children: [] });
          lastNode.children.push({ val: remain, children: [] });
        } else {
          // split leaf node
          let lastLetterBeforeRemain = word[word.lastIndexOf(remain) - 1];
          let { former, latter } = splitFromLast(
            lastNode.val,
            lastLetterBeforeRemain
          );

          lastNode.val = former;
          lastNode.children.push({ val: latter, children: [] });
          lastNode.children.push({ val: remain, children: [] });
        }
      } else {
        // split non-leaf node
        let lastLetterBeforeRemain = word[word.lastIndexOf(remain) - 1];
        let { former, latter } = splitFromLast(
          lastNode.val,
          lastLetterBeforeRemain
        );

        let newSplited = { val: latter, children: lastNode.children };
        lastNode.val = former;
        lastNode.children = [];
        lastNode.children.push(newSplited);
        lastNode.children.push({ val: remain, children: [] });
      }
    }
  }
}

// see https://en.wikipedia.org/wiki/Radix_tree#/media/File:Patricia_trie.svg
let tree = new RadixTrie();
tree.insert("romane");
tree.insert("romanus");
tree.insert("romulus");
tree.insert("rubens");
tree.insert("ruber");
tree.insert("rubicon");
tree.insert("rubicundus");

printTree(tree.root);
