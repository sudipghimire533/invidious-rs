pub trait GetRef<'a, Item> {
    fn get_ref() -> &'a Item;
}

pub trait GetOwned<Item> {
    fn get_owned() -> Item;
}
