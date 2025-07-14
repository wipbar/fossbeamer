{
  callPackage,
  defaultCrateOverrides,
  lib,
  wrapGAppsHook3,
  glib-networking,
  gst_all_1,
}:
(callPackage ../Cargo.nix {
  defaultCrateOverrides = defaultCrateOverrides // {
    fossbeamer = prev: {
      src = lib.fileset.toSource rec {
        root = prev.src.origSrc;
        fileset = (
          lib.fileset.intersection (lib.fileset.fromSource root) # We build our final fileset from the original src
            (lib.fileset.fileFilter (f: f.hasExt "rs") root)
        );
      };

      nativeBuildInputs = [
        wrapGAppsHook3
      ];

      buildInputs =
        with gst_all_1;
        [
          glib-networking
        ]
        ++ (with gst_all_1; [
          gstreamer
          gst-plugins-base
          gst-plugins-good
          gst-plugins-bad
          gst-libav
        ]);
    };
  };
}).rootCrate.build.override
  {
    runTests = true;
  }
