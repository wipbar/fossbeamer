self: pkgs: rec {
  fossbeamer_crates = import ../Cargo.nix {
    inherit pkgs;
    nixpkgs = pkgs.path;

    defaultCrateOverrides =
      let
        lib = pkgs.lib;
        # Filters the given source, only keeping files related to the build, preventing unnecessary rebuilds.
        # Includes src in the root, all other .rs files, as well as Cargo.toml.
        # Additional files to be included can be specified in extraFileset.
        filterRustCrateSrc =
          { root # The original src
          , extraFileset ? null # Additional filesets to include (e.g. test fixtures)
          }:
          lib.fileset.toSource {
            inherit root;
            fileset = (lib.fileset.intersection
              (lib.fileset.fromSource root) # We build our final fileset from the original src
              (lib.fileset.unions ([
                (root + "/src")
                (lib.fileset.fileFilter (f: f.hasExt "rs") root)
                # We assume that every Rust crate will at a minimum have .rs files and a Cargo.toml
                (lib.fileset.fileFilter (f: f.name == "Cargo.toml") root)
              ] ++ lib.optional (extraFileset != null) extraFileset)));
          };
      in
      pkgs.defaultCrateOverrides // {
        fossbeamer = prev: {
          src = filterRustCrateSrc { root = prev.src.origSrc; };

          nativeBuildInputs = [ pkgs.wrapGAppsHook ];
          buildInputs = with pkgs; [
            glib-networking
          ] ++ (with gst_all_1; [
            gstreamer
            gst-plugins-base
            gst-plugins-good
            gst-plugins-bad
            gst-libav
          ]);
        };
      };
  };

  fossbeamer = fossbeamer_crates.rootCrate.build.override {
    runTests = true;
  };
}
