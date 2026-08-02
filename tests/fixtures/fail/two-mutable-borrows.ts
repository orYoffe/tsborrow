type Owned<T> = T;

const queue: Owned<Queue> = createQueue();
const first = borrowMut(queue);
const second = borrowMut(queue);
first.push("a");
second.push("b");
