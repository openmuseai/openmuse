// java public final class KmoPair<A, B> {
pub struct KmoPair<A,B>{
    // java public final A first;
    pub first:A,
    // java public final B second;
    pub second:B,
}
impl<A,B>KmoPair<A,B>{
    // java private KmoPair(A first, B second) { this.first = first; this.second = second; }
    fn new(first:A,second:B)->Self{Self{first,second}}
    // java public static <A, B> KmoPair<A, B> of(A first, B second) { return new KmoPair<A, B>(first, second); }
    pub fn of(first:A,second:B)->Self{Self::new(first,second)}
}
