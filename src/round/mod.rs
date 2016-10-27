// Must define mod because I want to use folders as a package.
pub mod round;

// Somehow rust doubles the acces to Structs as round::round::Round.
// To prevent these and to use as round::Round we have to define "use" expession.
pub use self::round::Round;
