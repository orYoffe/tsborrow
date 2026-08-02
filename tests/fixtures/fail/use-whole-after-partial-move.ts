type Owned<T> = T;

const user: Owned<User> = loadUser();
const avatar = move(user.profile.avatar);
save(user);
upload(avatar);
