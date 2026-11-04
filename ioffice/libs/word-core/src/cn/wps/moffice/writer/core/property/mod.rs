pub mod IPropertyCalculate;
/* M3-10u: PropertyCalculateImpl and StyleCache compile independently.
 * PropertyCalculateFactory remains outside the product graph: its Java
 * WeakHashMap<Document,...> and WeakHashMap<TextDocument,...> identity/lifetime
 * behavior cannot be represented by the current empty WeakHashMap stub.
 * The live KStyles updateStyleCache stand-in is separately tracked; do not
 * claim a connected product property calculator until that route is replaced. */
// pub mod PropertyCalculateFactory;
pub mod StyleCache;
pub mod PropertyCalculateImpl;
