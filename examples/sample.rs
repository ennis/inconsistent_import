use inconsistent_import::*;
use image;

// thread 'main' panicked at 'src\librustc_resolve\resolve_imports.rs:970: inconsistent resolution for an import', src\librustc\util\bug.rs:47:26
//
// if the glob import is replaced by `inconsistent_import::image`, the compiler (correctly) emits
//      error[E0252]: the name `image` is defined multiple times
//
// if the glob import is moved after, the code compiles

fn main()
{
}
