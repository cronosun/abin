* Use https://github.com/tokio-rs/loom for tests.
* ~Improve the VecCapShrink~
* ~add "into_sync" for types~.
* cleanup BinData dings / TODOs
* ~no-allocation-guarantee tests~.
* ~Make sure we implement all interfaces from Bytes (e.g. AsRef)~ 
* Make sure we implement all interfaces from String / &string (die wesenltichen)
* Serde (kann man de-serializen into owned ohne allocation?)
* Cow
* Bin.try_merge()
* Vielleicht gemeinsame interfaces für die construction machen, wie `FromStatic<T=Bin>` oder `FromVec<T=SyncBin>` oder `FromVecShrink<T=Bin>` `CopyFromSlice` oder so was...
* Vielleicht ein Pure Arc / Pure Vec / Pure Rc machen welche nichts (wie shrinken und Empty/StackBin) machen... Dann diese wrappen?... nur so ne idee..