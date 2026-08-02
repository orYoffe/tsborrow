type Owned<T> = T;

const config: Owned<Config> = loadConfig();
const view = borrow(config);
endBorrow(view);
view.read();
