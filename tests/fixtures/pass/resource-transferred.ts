type Resource<T> = T;

const socket: Resource<Socket> = connect();
const workerSocket = move(socket);
workerSocket.send("hello");
