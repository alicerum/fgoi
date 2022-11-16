# fgoi

Fgoi is a simple sorter for go imports written in rust.

## Why did you do that??

Goimports is not exceptionally good with organizing blocks of
imports unless those imports are already organized by somebody. There are weird
artifacts that goimports produces that require manual organizing and I just don't
like to do that.

## Why fgoi?

Goimports is already taken, so I had to come up with some other name. Fgoi can be
read as `ffff goimports`, or maybe `fast goimports` (it is written in rust so it
has to be very fast, right? right?..), or whatever other way you like.

## Why rust?

Because I like it.

# How do I use thing bloody thing?

Very easy! Let me quote the help page of the app:

```
Usage: fgoi [OPTIONS] [FILES]...

Arguments:
  [FILES]...  

Options:
  -p, --package <PACKAGE>  Packages that need to be sorted separately from all the rest
  -h, --help               Print help information
```

* `FILES` - list of files (or directories) to be organized
  * Files should have `*.go` format
  * Directories will be traversed recursively in search of go files
* `-p` - [multiple] package masks to be separated in separate blocks

Every import in the file will be compared with the package mask provided in the
`-p` key by the user, and if said import begins with whatever is in the arg,
this import will go in its own block. With other imports that suit the mask.

## Example!

Let's say we have the `main.go` file with some messed up imports.

``` go
import (
	pkgname "github.com/alice/pakcage"
	
	"net/http"
	"core"
	
	"github.com/some/package"
	"github.com/alice/pkg2"
)
```

Then we run command on this file:

```
fgoi -p 'github.com/alice' main.go
```

And voila, our imports are all fancy and sorted!

``` go
import (
	"core"
	"net/http"
	
	"github.com/some/package"
	
	pkgname "github.com/alice/pakcage"
	"github.com/alice/pkg2"
)
```

Phew, this looks much better, and my OCD is finally happy.
