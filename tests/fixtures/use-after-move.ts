type Owned<T> = T;

const connection: Owned<Connection> = openConnection();
const transaction = move(connection);
connection.query("SELECT 1");
