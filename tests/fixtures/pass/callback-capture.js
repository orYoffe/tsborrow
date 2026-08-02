/** @owned */
const socket = connect();
const view = borrow(socket);
setTimeout(() => view.read(), 100);
