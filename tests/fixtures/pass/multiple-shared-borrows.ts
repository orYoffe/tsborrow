type Owned<T> = T;

const document: Owned<Document> = openDocument();
const left = borrow(document);
const right = borrow(document);
left.read();
right.read();
