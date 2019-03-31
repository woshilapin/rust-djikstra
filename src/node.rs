#[derive(Clone)]
pub struct Node {
    pub id: usize,
    pub connections: Vec<Link>,
}

#[derive(Clone)]
pub struct Link {
    pub weight: u64,
    pub to: usize,
}