pub struct BinaryTree<T> {
    pub value: T,
    pub left: Option<Box<BinaryTree<T>>>,
    pub right: Option<Box<BinaryTree<T>>>,
}

impl<T> BinaryTree<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    pub fn left(mut self, value: BinaryTree<T>) -> Self {
        self.left = Some(Box::new(value));
        self
    }

    pub fn right(mut self, value: BinaryTree<T>) -> Self {
        self.right = Some(Box::new(value));
        self
    }
}

pub fn encode<'a>(text: &'a str, tree: &Option<Box<BinaryTree<char>>>) -> String {
    // search in the binary tree for a character in text
    // if the character is found, every left leaf turns into a dot `.`,
    // every right leaf turns into a dash `-`
    // spaces are spaces and ignored
    
    // make sure the input string is all lowercase.
    // our binary tree only contains lower-case characters
    let text = text.to_lowercase();

    let chars = text.chars();

    let mut output = vec![];

    for c in chars {
        let mut letter = vec![];
        get_morse_code(tree, c, &mut letter);
        output.extend_from_slice(&letter);
        output.push(' ');
    }

    output.into_iter().collect()
}

pub fn decode<'a>(morse: &'a str, tree: &Option<Box<BinaryTree<char>>>) -> String {
    // The output string has its words split by 2 spaces,
    // individual letters are split by a single space.

    // split morse by 2 spaces
    let words = morse.split("  ").collect::<Vec<_>>();
    let mut output = vec![];

    // for every word
    for w in words {
        // split out the letters
        let letters = w.split(" ").collect::<Vec<_>>();
        for l in letters {
            // feed the dashes-and-dots of a single letter into get_letter
            if let Some(result) = get_letter(&l, tree) {
                // add the result to the output
                output.push(result);
            }
        }

        // after every word, add a space
        output.push(' ');
    }

    // convert the vec collection to a string
    output.into_iter().collect::<String>()
}

fn get_letter(morse: &str, node: &Option<Box<BinaryTree<char>>>) -> Option<char> {
    let mut node = node;
    for c in morse.chars() {
        let Some(n) = node else {
            return None; // invalid morse sequence?
        };
        match c {
            '.' => node = &n.left, // go left
            '-' => node = &n.right, // go right
            c => panic!("Unexpected character '{}', expected '.' or '-'", c)
        }
    }
    let Some(n) = node else {
        return None; // nothing found
    };
    Some(n.value)
}

fn get_morse_code(node: &Option<Box<BinaryTree<char>>>, c: char, output: &mut Vec<char>) -> bool {
    let Some(node) = node else {
        return false;
    };

    if node.value == c {
        return true;
    }

    if get_morse_code(&node.left, c, output) {
        output.insert(0, '.');
        return true;
    }

    if get_morse_code(&node.right, c, output) {
        output.insert(0, '-');
        return true;
    }

    return false;
}

fn main() {
    // this tree should probably be built at compile time..
    // not sure how to do that though.
    let tree = BinaryTree::new(' ')
        .left(
            BinaryTree::new('e')
                .left(
                    BinaryTree::new('i')
                        .left(
                            BinaryTree::new('s')
                                .left(
                                    BinaryTree::new('h')
                                        .left(BinaryTree::new('5'))
                                        .right(BinaryTree::new('4')),
                                )
                                .right(BinaryTree::new('v').right(BinaryTree::new('3'))),
                        )
                        .right(
                            BinaryTree::new('u')
                                .left(BinaryTree::new('f'))
                                .right(BinaryTree::new('?').right(BinaryTree::new('2'))),
                        ),
                )
                .right(
                    BinaryTree::new('a')
                        .left(
                            BinaryTree::new('r')
                                .left(BinaryTree::new('l'))
                                .right(BinaryTree::new('?').left(BinaryTree::new('+'))),
                        )
                        .right(
                            BinaryTree::new('w')
                                .left(BinaryTree::new('p'))
                                .right(BinaryTree::new('j').right(BinaryTree::new('1'))),
                        ),
                ),
        )
        .right(
            BinaryTree::new('t')
                .left(
                    BinaryTree::new('n')
                        .left(
                            BinaryTree::new('d')
                                .left(
                                    BinaryTree::new('b')
                                        .left(BinaryTree::new('6'))
                                        .right(BinaryTree::new('=')),
                                )
                                .right(BinaryTree::new('x').left(BinaryTree::new('/'))),
                        )
                        .right(
                            BinaryTree::new('k')
                                .left(BinaryTree::new('c'))
                                .right(BinaryTree::new('y')),
                        ),
                )
                .right(
                    BinaryTree::new('m')
                        .left(
                            BinaryTree::new('g')
                                .left(BinaryTree::new('z').left(BinaryTree::new('7')))
                                .right(BinaryTree::new('q')),
                        )
                        .right(
                            BinaryTree::new('o')
                                .left(BinaryTree::new('?').left(BinaryTree::new('8')))
                                .right(
                                    BinaryTree::new('?')
                                        .left(BinaryTree::new('9'))
                                        .right(BinaryTree::new('0')),
                                ),
                        ),
                ),
        );

    // wrap it in a Option<Box<T>>
    // as that's what our encode and decode
    // functions expect
    let tree = Some(Box::new(tree));

    let output = encode("hello world", &tree);
    println!("encoded: {}", output);
    println!("decoded: {}", decode(&output, &tree));
}

