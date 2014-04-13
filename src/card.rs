#[deriving(Decodable,Show)]
pub struct CardList {
    object: ~str,
    has_more: bool,
    url: ~str,
    data: ~[Card]
}

#[deriving(Decodable,Show)]
pub struct Card;
